// src/main.rs
// API principal do FC Data - Sistema de consulta de dados do FC

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

mod auth;
mod config;
mod database;
mod errors;
mod handlers;
mod models;

use crate::config::Settings;
use crate::database::DatabasePools;
use crate::handlers::{auth_handlers, data_handlers};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar logger
    env_logger::init();

    println!("🚀 Iniciando FC Data API...");

    // Carregar configurações (que agora FORÇA o carregamento do .env)
    let settings = match Settings::from_env() {
        Ok(s) => {
            log::info!("✅ Configurações carregadas com sucesso");
            log::info!("  - Admin user: {}", s.admin.username);
            log::info!("  - Server: {}:{}", s.server.host, s.server.port);
            log::info!("  - API Prefix: {}", s.api.prefix);
            log::info!("  - CORS origins: {:?}", s.cors.allowed_origins);
            s
        },
        Err(e) => {
            log::error!("❌ ERRO ao carregar configurações: {:?}", e);
            panic!("Não foi possível carregar as configurações: {:?}", e);
        }
    };

    log::info!("📊 Conectando aos bancos de dados...");
    
    // Criar pools de conexão para os 3 bancos
    let db_pools = DatabasePools::new().await
        .expect("Erro ao criar pools de conexões");
    
    log::info!("✅ Pools de conexões criados com sucesso");
    log::info!("  - PostgreSQL FC: ✓");
    log::info!("  - SQL Server Portal: ✓");
    log::info!("  - SQL Server Protheus: ✓");

    let bind_address = format!("{}:{}", settings.server.host, settings.server.port);
    log::info!("🌐 Servidor rodando em http://{}", bind_address);
    log::info!("📍 API disponível em http://{}{}", bind_address, settings.api.prefix);

    // Criar servidor HTTP
    let cors_origins = settings.cors.allowed_origins.clone();
    
    HttpServer::new(move || {
        // Configurar CORS
        let cors = Cors::default()
            .allowed_origin_fn({
                let origins = cors_origins.clone();
                move |origin, _req_head| {
                    origins.iter()
                        .any(|allowed| origin.as_bytes() == allowed.as_bytes())
                }
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Authorization", "Content-Type"])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(db_pools.clone()))
            .app_data(web::Data::new(db_pools.postgres_fc.clone()))
            .app_data(web::Data::new(settings.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope(&settings.api.prefix)
                    // Rotas de autenticação
                    .route("/auth/login", web::post().to(auth_handlers::login))
                    
                    // Rota de validação (protegida por JWT)
                    .service(
                        web::resource("/auth/validate")
                            .wrap(auth::JwtMiddleware)
                            .route(web::get().to(auth_handlers::validate_token))
                    )
                    
                    // Rotas de dados (protegidas por JWT)
                    .service(
                        web::scope("/data")
                            .wrap(auth::JwtMiddleware)
                            .route("/vendas", web::get().to(data_handlers::get_vendas))
                            .route("/vendas/detalhes", web::get().to(data_handlers::get_vendas_detalhadas))
                            .route("/query", web::post().to(handlers::query_handlers::execute_custom_query))
                    )
                    
                    // Rotas do Portal (protegidas por JWT)
                    .service(
                        web::scope("/portal")
                            .wrap(auth::JwtMiddleware)
                            .route("/query", web::post().to(handlers::portal_handlers::query_portal))
                            .route("/produtos", web::get().to(handlers::portal_handlers::listar_produtos_por_grupo))
                            .route("/pedidos", web::post().to(handlers::pedido_handlers::criar_pedido))
                            .route("/pedidos/{id}", web::get().to(handlers::pedido_handlers::buscar_pedido))
                            .route("/pedidos/{id}", web::put().to(handlers::pedido_handlers::atualizar_pedido))
                            .route("/pedidos/{id}", web::delete().to(handlers::pedido_handlers::deletar_pedido))
                            .route("/pedidos/{id}/confirmar", web::post().to(handlers::pedido_handlers::confirmar_pedido))
                    )
                    
                    // Rotas do Protheus (protegidas por JWT)
                    .service(
                        web::scope("/protheus")
                            .wrap(auth::JwtMiddleware)
                            .route("/query", web::post().to(handlers::protheus_handlers::query_protheus))
                            .route("/pedidos/{numero}/status", web::get().to(handlers::protheus_handlers::status_pedido_protheus))
                    )
                    
                    // Rotas de Analytics (protegidas por JWT)
                    .service(
                        web::scope("/analytics")
                            .wrap(auth::JwtMiddleware)
                            .route("/cliente/{cnpj}/360", web::get().to(handlers::analytics_handlers::analytics_cliente_360))
                            .route("/produtos/{id}/correlacoes", web::get().to(handlers::analytics_handlers::correlacoes_produto))
                            .route("/recompra-inteligente", web::get().to(handlers::analytics_handlers::recompra_inteligente))
                            .route("/oportunidades-rede", web::get().to(handlers::analytics_handlers::oportunidades_rede))
                    )
                    
                    // Health check público
                    .route("/health", web::get().to(handlers::health_check))
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}