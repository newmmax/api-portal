//! 🔧 Helper Functions para Pedidos
//! 
//! Funções utilitárias para:
//! - Conversões de tipos SQL Server (tiberius)
//! - Transações manuais BEGIN/COMMIT
//! - Tratamento de erros com .map_err()

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use tiberius::{Query, Row};
use crate::errors::ApiError;
use bb8::PooledConnection;
use bb8_tiberius::ConnectionManager;

/// Helper para converter Option<&str> do tiberius de forma segura
pub fn get_optional_string(row: &Row, index: usize) -> Option<String> {
    row.get::<&str, _>(index).map(|s| s.to_string())
}

/// Helper para converter Option<&str> com valor padrão
pub fn get_string_with_default(row: &Row, index: usize, default: &str) -> String {
    row.get::<&str, _>(index)
        .map(|s| s.to_string())
        .unwrap_or_else(|| default.to_string())
}

/// Helper para converter Decimal para f64 usando ToPrimitive
pub fn decimal_to_f64(decimal: Decimal) -> f64 {
    decimal.to_f64().unwrap_or(0.0)
}

/// Helper para converter Option<Decimal> para f64
pub fn optional_decimal_to_f64(decimal: Option<Decimal>) -> f64 {
    decimal
        .unwrap_or(Decimal::ZERO)
        .to_f64()
        .unwrap_or(0.0)
}

/// Helper para executar transação manual BEGIN
pub async fn begin_transaction(
    conn: &mut PooledConnection<'_, ConnectionManager>
) -> Result<(), ApiError> {
    Query::new("BEGIN TRANSACTION")
        .execute(conn)
        .await
        .map_err(|e| ApiError::Database(format!("Erro ao iniciar transação: {}", e)))?;
    Ok(())
}

/// Helper para executar COMMIT
pub async fn commit_transaction(
    conn: &mut PooledConnection<'_, ConnectionManager>
) -> Result<(), ApiError> {
    Query::new("COMMIT TRANSACTION")
        .execute(conn)
        .await
        .map_err(|e| ApiError::Database(format!("Erro ao confirmar transação: {}", e)))?;
    Ok(())
}

/// Helper para executar ROLLBACK
pub async fn rollback_transaction(
    conn: &mut PooledConnection<'_, ConnectionManager>
) -> Result<(), ApiError> {
    Query::new("ROLLBACK TRANSACTION")
        .execute(conn)
        .await
        .map_err(|e| ApiError::Database(format!("Erro ao desfazer transação: {}", e)))?;
    Ok(())
}

/// Helper para executar ROLLBACK em caso de erro (ignora falhas do rollback)
pub async fn safe_rollback(conn: &mut PooledConnection<'_, ConnectionManager>) {
    if let Err(e) = rollback_transaction(conn).await {
        log::error!("Falha ao executar rollback: {}", e);
    }
}

/// Macro para simplificar .map_err() em queries tiberius
macro_rules! map_tiberius_error {
    ($result:expr, $context:expr) => {
        $result.map_err(|e| ApiError::Database(format!("{}: {}", $context, e)))
    };
}

pub(crate) use map_tiberius_error;
