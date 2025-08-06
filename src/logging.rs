use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;
use serde_json::json;
use std::sync::Mutex;

/// Sistema de logging específico para Cards Analytics
pub struct CardsLogger {
    enabled: bool,
    log_file: String,
}

impl CardsLogger {
    pub fn new() -> Self {
        let enabled = std::env::var("ENABLE_DEBUG_LOGS")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);
            
        let log_file = std::env::var("DEBUG_LOG_FILE")
            .unwrap_or_else(|_| "cards_debug.log".to_string());

        CardsLogger { enabled, log_file }
    }

    /// Log detalhado para início de requisição de Card
    pub fn log_card_request(&self, card_name: &str, cnpj: &str, params: &str) {
        if !self.enabled {
            return;
        }

        let log_entry = json!({
            "timestamp": Utc::now().to_rfc3339(),
            "level": "INFO",
            "card": card_name,
            "event": "REQUEST_START",
            "cnpj": cnpj,
            "params": params,
            "thread": format!("{:?}", std::thread::current().id())
        });

        self.write_log(&log_entry.to_string());
    }

    /// Log para query SQL sendo executada
    #[allow(dead_code)] // Funcionalidade futura para debugging avançado
    pub fn log_sql_execution(&self, card_name: &str, cnpj: &str, sql: &str, params: &[String]) {
        if !self.enabled {
            return;
        }

        let log_entry = json!({
            "timestamp": Utc::now().to_rfc3339(),
            "level": "DEBUG",
            "card": card_name,
            "event": "SQL_EXECUTION",
            "cnpj": cnpj,
            "sql": sql.chars().take(500).collect::<String>(), // Primeiros 500 chars
            "params": params,
            "sql_length": sql.len(),
            "thread": format!("{:?}", std::thread::current().id())
        });

        self.write_log(&log_entry.to_string());
    }

    /// Log para erros específicos
    pub fn log_error(&self, card_name: &str, cnpj: &str, error: &str, context: &str) {
        if !self.enabled {
            return;
        }

        let log_entry = json!({
            "timestamp": Utc::now().to_rfc3339(),
            "level": "ERROR",
            "card": card_name,
            "event": "ERROR",
            "cnpj": cnpj,
            "error": error,
            "context": context,
            "thread": format!("{:?}", std::thread::current().id())
        });

        self.write_log(&log_entry.to_string());
        
        // Também log no sistema principal
        log::error!("[{}] {} - {}: {}", card_name, cnpj, context, error);
    }

    /// Log para normalização de CNPJ
    pub fn log_cnpj_normalization(&self, cnpj_original: &str, cnpj_formatado: &str) {
        if !self.enabled {
            return;
        }

        let log_entry = json!({
            "timestamp": Utc::now().to_rfc3339(),
            "level": "DEBUG",
            "event": "CNPJ_NORMALIZATION",
            "cnpj_original": cnpj_original,
            "cnpj_formatado": cnpj_formatado,
            "was_normalized": cnpj_original != cnpj_formatado,
            "thread": format!("{:?}", std::thread::current().id())
        });

        self.write_log(&log_entry.to_string());
    }

    /// Log para conexão com banco
    #[allow(dead_code)] // Método útil para debug de conexões - uso futuro
    pub fn log_database_connection(&self, database: &str, success: bool, error: Option<&str>) {
        if !self.enabled {
            return;
        }

        let log_entry = json!({
            "timestamp": Utc::now().to_rfc3339(),
            "level": if success { "DEBUG" } else { "ERROR" },
            "event": "DATABASE_CONNECTION",
            "database": database,
            "success": success,
            "error": error,
            "thread": format!("{:?}", std::thread::current().id())
        });

        self.write_log(&log_entry.to_string());
    }

    /// Escrever log no arquivo
    fn write_log(&self, log_entry: &str) {
        let mut file = match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)
        {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Erro ao abrir arquivo de log {}: {}", self.log_file, e);
                return;
            }
        };

        if let Err(e) = writeln!(file, "{}", log_entry) {
            eprintln!("Erro ao escrever no log: {}", e);
        }
    }

    /// Ler últimas N linhas do log
    pub fn read_recent_logs(&self, lines: usize) -> Result<Vec<String>, std::io::Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(&self.log_file)?;
        let reader = BufReader::new(file);
        let all_lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;
        
        let start = if all_lines.len() > lines {
            all_lines.len() - lines
        } else {
            0
        };

        Ok(all_lines[start..].to_vec())
    }

    /// Limpar logs antigos (manter apenas N linhas)
    pub fn rotate_logs(&self, max_lines: usize) -> Result<(), std::io::Error> {
        if !self.enabled {
            return Ok(());
        }

        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(&self.log_file)?;
        let reader = BufReader::new(file);
        let all_lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;

        if all_lines.len() > max_lines {
            let keep_lines = &all_lines[(all_lines.len() - max_lines)..];
            std::fs::write(&self.log_file, keep_lines.join("\n"))?;
        }

        Ok(())
    }

    /// Verificar se logging está habilitado
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get path do arquivo de log
    pub fn get_log_file_path(&self) -> &str {
        &self.log_file
    }
}

// Instância global do logger (thread-safe)
lazy_static::lazy_static! {
    pub static ref CARDS_LOGGER: Mutex<CardsLogger> = Mutex::new(CardsLogger::new());
}

/// Macro para facilitar o uso do logger
#[macro_export]
macro_rules! cards_log {
    (request, $card:expr, $cnpj:expr, $params:expr) => {
        if let Ok(logger) = crate::logging::CARDS_LOGGER.lock() {
            logger.log_card_request($card, $cnpj, $params);
        }
    };
    
    (sql, $card:expr, $cnpj:expr, $sql:expr, $params:expr) => {
        if let Ok(logger) = crate::logging::CARDS_LOGGER.lock() {
            logger.log_sql_execution($card, $cnpj, $sql, $params);
        }
    };
    
    (error, $card:expr, $cnpj:expr, $error:expr, $context:expr) => {
        if let Ok(logger) = crate::logging::CARDS_LOGGER.lock() {
            logger.log_error($card, $cnpj, $error, $context);
        }
    };
    
    (cnpj, $original:expr, $formatted:expr) => {
        if let Ok(logger) = crate::logging::CARDS_LOGGER.lock() {
            logger.log_cnpj_normalization($original, $formatted);
        }
    };
    
    (db, $database:expr, $success:expr, $error:expr) => {
        if let Ok(logger) = crate::logging::CARDS_LOGGER.lock() {
            logger.log_database_connection($database, $success, $error);
        }
    };
}
