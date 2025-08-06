use bb8_tiberius::ConnectionManager;
#[allow(unused_imports)] // Usado em analytics_handlers com caminho completo bb8::PooledConnection
use bb8::{Pool, PooledConnection};
use deadpool_postgres::{Config as PgConfig, Pool as PgPool, Runtime};
use std::env;
use thiserror::Error;
use tokio_postgres::NoTls;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("PostgreSQL connection error: {0}")]
    PostgresError(#[from] deadpool_postgres::PoolError),
    
    #[error("SQL Server connection error: {0}")]
    SqlServerError(#[from] bb8::RunError<tiberius::error::Error>),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type SqlServerPool = Pool<ConnectionManager>;
// Removido SqlServerConnection type alias não usado

#[derive(Clone)]
pub struct DatabasePools {
    pub postgres_fc: PgPool,
    pub sqlserver_portal: SqlServerPool,
    pub sqlserver_protheus: SqlServerPool,
}

impl DatabasePools {
    pub async fn new() -> Result<Self, DatabaseError> {
        // PostgreSQL FC Data
        let postgres_fc = create_postgres_pool()?;
        
        // SQL Server Portal
        let portal_conn_str = env::var("PORTAL_CONNECTION_STRING")
            .map_err(|_| DatabaseError::ConfigError("PORTAL_CONNECTION_STRING not set".to_string()))?;
        let sqlserver_portal = create_sqlserver_pool(&portal_conn_str).await?;
        
        // SQL Server Protheus
        let protheus_conn_str = env::var("PROTHEUS_CONNECTION_STRING")
            .map_err(|_| DatabaseError::ConfigError("PROTHEUS_CONNECTION_STRING not set".to_string()))?;
        let sqlserver_protheus = create_sqlserver_pool(&protheus_conn_str).await?;
        
        Ok(DatabasePools {
            postgres_fc,
            sqlserver_portal,
            sqlserver_protheus,
        })
    }
}

fn create_postgres_pool() -> Result<PgPool, DatabaseError> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| DatabaseError::ConfigError("DATABASE_URL not set".to_string()))?;
    
    let mut cfg = PgConfig::new();
    cfg.url = Some(database_url);
    
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|e| DatabaseError::ConfigError(format!("Failed to create PostgreSQL pool: {}", e)))
}

async fn create_sqlserver_pool(connection_string: &str) -> Result<SqlServerPool, DatabaseError> {
    let config = tiberius::Config::from_ado_string(connection_string)
        .map_err(|e| DatabaseError::ConfigError(format!("Invalid SQL Server connection string: {}", e)))?;
    
    let mgr = ConnectionManager::new(config);
    
    Pool::builder()
        .max_size(10)
        .build(mgr)
        .await
        .map_err(|e| DatabaseError::ConfigError(format!("Failed to create SQL Server pool: {}", e)))
}
