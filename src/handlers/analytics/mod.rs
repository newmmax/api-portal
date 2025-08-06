//! 🎯 Analytics Handlers - Modularizado para < 500 linhas por arquivo
//! 
//! Cards Analytics do Sistema FC Data API:
//! - Card 01: Recompra Inteligente  
//! - Card 02: Oportunidades na Rede
//! - Novos: Pedido Analysis, Export, Efetividade

// Modules
pub mod recompra;
pub mod oportunidades;
pub mod pedido_analysis;
pub mod export;
pub mod efetividade;
pub mod helpers;

// Re-exports for main.rs compatibility
pub use recompra::recompra_inteligente;
pub use oportunidades::oportunidades_rede;
pub use pedido_analysis::analisar_pedido_oportunidades;
pub use export::exportar_relatorio;
pub use efetividade::buscar_efetividade_sugestoes;

// Legacy compatibility
pub use helpers::{analytics_cliente_360, correlacoes_produto};
