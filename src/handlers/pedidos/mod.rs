//! 🛒 Pedidos Handlers - Modularizado para < 500 linhas por arquivo
//!
//! Sistema completo de pedidos:
//! - CRUD básico de pedidos
//! - Geração com oportunidades (Card integration)
//! - Tracking de sugestões aceitas/rejeitadas

// Modules  
pub mod crud;
pub mod geracao;
pub mod tracking;

// Re-exports for main.rs compatibility  
pub use crud::{criar_pedido, buscar_pedido, atualizar_pedido, deletar_pedido, confirmar_pedido};
pub use geracao::gerar_pedido_com_oportunidades;
pub use tracking::marcar_item_sugestao;
