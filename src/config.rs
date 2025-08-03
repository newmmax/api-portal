// src/config.rs - VERSÃO CORRIGIDA QUE FORÇA LEITURA DO .ENV
// Configurações da aplicação

use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub cors: CorsConfig,
    pub api: ApiConfig,
    pub admin: AdminConfig,
    pub portal_database: SqlServerConfig,
    pub protheus_database: SqlServerConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiConfig {
    pub prefix: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdminConfig {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SqlServerConfig {
    pub connection_string: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}

impl Settings {
    pub fn from_env() -> Result<Self, env::VarError> {
        // FORCE LOAD .ENV - Tentar vários caminhos
        println!("🔍 BUSCANDO .env files...");
        
        // Caminho 1: Diretório atual
        if Path::new(".env").exists() {
            println!("✅ Encontrado .env no diretório atual");
            match dotenv::from_filename(".env") {
                Ok(_) => println!("✅ .env carregado com sucesso"),
                Err(e) => println!("❌ Erro ao carregar .env: {:?}", e),
            }
        } else {
            println!("❌ .env não encontrado no diretório atual");
        }
        
        // Caminho 2: Diretório do executável
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let env_path = exe_dir.join(".env");
                if env_path.exists() {
                    println!("✅ Encontrado .env no diretório do executável: {:?}", env_path);
                    match dotenv::from_path(&env_path) {
                        Ok(_) => println!("✅ .env do executável carregado com sucesso"),
                        Err(e) => println!("❌ Erro ao carregar .env do executável: {:?}", e),
                    }
                } else {
                    println!("❌ .env não encontrado no diretório do executável: {:?}", env_path);
                }
            }
        }
        
        // Listar TODAS as variáveis de ambiente relacionadas
        println!("🔍 VARIÁVEIS DE AMBIENTE CARREGADAS:");
        let vars_to_check = [
            "DATABASE_URL", "ADMIN_USERNAME", "ADMIN_PASSWORD", 
            "PORTAL_CONNECTION_STRING", "SERVER_PORT"
        ];
        
        for var in &vars_to_check {
            match env::var(var) {
                Ok(value) => {
                    if var.contains("PASSWORD") {
                        println!("  {} = [REDACTED - {} chars]", var, value.len());
                    } else {
                        println!("  {} = {}", var, value);
                    }
                }
                Err(_) => println!("  {} = [NÃO ENCONTRADA]", var),
            }
        }

        // Parse DATABASE_URL para componentes individuais
        let database_url = env::var("DATABASE_URL")?;
        let db_config = Self::parse_database_url(&database_url)?;

        Ok(Settings {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| {
                    println!("⚠️ SERVER_HOST não encontrado, usando padrão");
                    "127.0.0.1".to_string()
                }),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| {
                        println!("⚠️ SERVER_PORT não encontrado, usando padrão 8080");
                        "8080".to_string()
                    })
                    .parse()
                    .unwrap_or(8080),
            },
            database: db_config,
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET")
                    .unwrap_or_else(|_| {
                        println!("⚠️ JWT_SECRET não encontrado, usando padrão");
                        "default_secret_change_in_production".to_string()
                    }),
                expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                    .unwrap_or_else(|_| "24".to_string())
                    .parse()
                    .unwrap_or(24),
            },
            cors: CorsConfig {
                allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| {
                        println!("⚠️ CORS_ALLOWED_ORIGINS não encontrado, usando padrão");
                        "https://conexao.artesanalfarmacia.com.br".to_string()
                    })
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
            api: ApiConfig {
                prefix: env::var("API_PREFIX")
                    .unwrap_or_else(|_| {
                        println!("⚠️ API_PREFIX não encontrado, usando padrão");
                        "/services/api1".to_string()
                    }),
            },
            admin: AdminConfig {
                username: env::var("ADMIN_USERNAME")
                    .unwrap_or_else(|_| {
                        println!("❌ ADMIN_USERNAME não encontrado no .env!");
                        "admin".to_string()
                    }),
                password: env::var("ADMIN_PASSWORD")
                    .unwrap_or_else(|_| {
                        println!("❌ ADMIN_PASSWORD não encontrado no .env!");
                        "admin123".to_string()
                    }),
            },
            portal_database: SqlServerConfig {
                connection_string: env::var("PORTAL_CONNECTION_STRING")
                    .unwrap_or_else(|_| {
                        println!("⚠️ PORTAL_CONNECTION_STRING não encontrado");
                        "".to_string()
                    }),
                host: env::var("PORTAL_DATABASE_HOST")
                    .unwrap_or_else(|_| "localhost".to_string()),
                port: env::var("PORTAL_DATABASE_PORT")
                    .unwrap_or_else(|_| "1433".to_string())
                    .parse()
                    .unwrap_or(1433),
                database: env::var("PORTAL_DATABASE_NAME")
                    .unwrap_or_else(|_| "sys_pedidos".to_string()),
                user: env::var("PORTAL_DATABASE_USER")
                    .unwrap_or_else(|_| "sa".to_string()),
                password: env::var("PORTAL_DATABASE_PASS")
                    .unwrap_or_else(|_| "".to_string()),
            },
            protheus_database: SqlServerConfig {
                connection_string: env::var("PROTHEUS_CONNECTION_STRING")
                    .unwrap_or_else(|_| "".to_string()),
                host: env::var("PROTHEUS_DATABASE_HOST")
                    .unwrap_or_else(|_| "localhost".to_string()),
                port: env::var("PROTHEUS_DATABASE_PORT")
                    .unwrap_or_else(|_| "1433".to_string())
                    .parse()
                    .unwrap_or(1433),
                database: env::var("PROTHEUS_DATABASE_NAME")
                    .unwrap_or_else(|_| "SIGAOFC".to_string()),
                user: env::var("PROTHEUS_DATABASE_USER")
                    .unwrap_or_else(|_| "sa".to_string()),
                password: env::var("PROTHEUS_DATABASE_PASS")
                    .unwrap_or_else(|_| "".to_string()),
            },
        })
    }

    fn parse_database_url(url: &str) -> Result<DatabaseConfig, env::VarError> {
        // postgres://user:password@host:port/dbname
        let parts: Vec<&str> = url.split("://").collect();
        if parts.len() != 2 || parts[0] != "postgres" {
            return Err(env::VarError::NotPresent);
        }

        let remaining = parts[1];
        let auth_and_rest: Vec<&str> = remaining.split('@').collect();
        if auth_and_rest.len() != 2 {
            return Err(env::VarError::NotPresent);
        }

        let auth_parts: Vec<&str> = auth_and_rest[0].split(':').collect();
        let host_and_db: Vec<&str> = auth_and_rest[1].split('/').collect();
        let host_parts: Vec<&str> = host_and_db[0].split(':').collect();

        Ok(DatabaseConfig {
            host: host_parts[0].to_string(),
            port: host_parts.get(1)
                .and_then(|p| p.parse().ok())
                .unwrap_or(5432),
            user: auth_parts[0].to_string(),
            password: auth_parts.get(1).unwrap_or(&"").to_string(),
            dbname: host_and_db.get(1).unwrap_or(&"postgres").to_string(),
        })
    }
}

// Implementação para deadpool-postgres
impl From<DatabaseConfig> for deadpool_postgres::Config {
    fn from(cfg: DatabaseConfig) -> Self {        let mut config = deadpool_postgres::Config::new();
        config.host = Some(cfg.host);
        config.port = Some(cfg.port);
        config.user = Some(cfg.user);
        config.password = Some(cfg.password);
        config.dbname = Some(cfg.dbname);
        config
    }
}
