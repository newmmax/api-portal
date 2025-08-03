// src/models.rs
// Modelos de dados da aplicação

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Estrutura para resumo de vendas
#[derive(Debug, Serialize, Deserialize)]
pub struct VendaResumo {
    pub empresa: Option<String>,
    pub cnpj: Option<String>,
    pub filial_codigo: Option<i32>,
    pub filial_nome: Option<String>,
    pub cupom: Option<i64>,
    pub data_venda: Option<NaiveDate>,
    pub cliente_codigo: Option<i32>,
    pub cliente_nome: Option<String>,
    pub vendedor_codigo: Option<i32>,
    pub vendedor_nome: Option<String>,
    pub total_itens: u32,
    pub valor_total: f64,
}

/// Estrutura detalhada de vendas com informações de produtos
#[derive(Debug, Serialize, Deserialize)]
pub struct VendaDetalhada {
    // Dados do cabeçalho
    pub empresa: Option<String>,
    pub cnpj: Option<String>,
    pub filial_codigo: Option<i32>,
    pub filial_nome: Option<String>,
    pub cupom: Option<i64>,
    pub data_venda: Option<NaiveDate>,
    pub data_emissao_nfce: Option<NaiveDate>,
    pub cliente_codigo: Option<i32>,
    pub cliente_nome: Option<String>,
    pub vendedor_codigo: Option<i32>,
    pub vendedor_nome: Option<String>,
    
    // Dados do item
    pub item_id: Option<i32>,
    pub produto_codigo: Option<i32>,
    pub produto_descricao: Option<String>,
    pub setor: Option<String>,
    pub quantidade: f64,
    pub preco_unitario: f64,
    pub valor_total: f64,
    pub valor_desconto: f64,
    pub valor_recebido: f64,
    pub preco_custo: Option<f64>,
    pub preco_compra: Option<f64>,
}

/// Parâmetros de filtro para consultas de vendas
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FiltrosVenda {
    pub data_inicio: Option<String>,
    pub data_fim: Option<String>,
    pub empresa: Option<String>,
    pub filial: Option<i32>,
    pub cliente: Option<i32>,
    pub vendedor: Option<i32>,
    pub produto: Option<String>,
    pub limite: Option<i64>,
    pub offset: Option<i64>,
}

/// Response padrão para erros
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

/// Response para health check
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
    pub database: String,
    pub timestamp: DateTime<Utc>,
}
// Adicionar aos models existentes

use rust_decimal::Decimal;

// ===== MODELS DO PORTAL DE PEDIDOS =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pedido {
    pub id: i32,
    pub cliente_id: i32,
    pub codigo_cliente: String,
    pub numero_pedido: Option<String>,
    pub loja_cliente: String,
    pub emissao: String,
    pub dt_envio: Option<String>,
    pub condicao_pagamento: Option<String>,
    pub tipo_pedido: Option<String>,
    pub tabela_precos: Option<String>,
    pub tipo_frete: Option<String>,
    pub mensagem: Option<String>,
    pub natureza: String,
    pub status_pedido: String,
    pub numero_nota_fiscal: Option<String>,
    pub transportadora: Option<String>,
    pub rastreio_carga: Option<String>,
    pub vendedor: Option<String>,
    pub integrado: bool,
    pub confirmado: bool,
    pub status_liberacao: Option<String>,
    pub tentativas_integracao: Option<i32>,
    pub regra_condicao_pagamento_id: i32,
    pub regra_frete_id: i32,
    pub nota: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub pedido_id: i32,
    pub produto_id: i32,
    pub quantidade: i32,
    pub preco_unitario: Decimal,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Produto {
    pub id: i32,
    pub codigo: String,
    pub descricao: String,
    pub tipo: Option<String>,
    pub unidade_medida: String,
    pub preco_unitario: Option<Decimal>,
    pub quantidade_minima_embalagem: i32,
    pub saldo: i32,
    pub estoque: Option<i32>,
    pub categoria_id: Option<i32>,
    pub b2_filial: Option<String>,
    pub status: bool,
    pub foto: Option<String>,
    pub grupo_venda: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecoProduto {
    pub id: i32,
    pub codigo_produto: String,
    pub grupo_venda: String,
    pub preco: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegraFrete {
    pub id: i32,
    pub valor_minimo: Decimal,
    pub descricao: String,
    pub codigo_protheus: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegraParcelamento {
    pub id: i32,
    pub valor_minimo: Decimal,
    pub valor_maximo: Option<Decimal>,
    pub codigo_protheus: String,
    pub descricao: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cliente {
    pub id: i32,
    pub codigo: String,
    pub loja: String,
    pub razao_social: String,
    pub nome_fantasia: Option<String>,
    pub cnpj: String,
    pub inscricao_estadual: Option<String>,
    pub endereco: Option<String>,
    pub bairro: Option<String>,
    pub cidade: Option<String>,
    pub estado: Option<String>,
    pub cep: Option<String>,
    pub telefone: Option<String>,
    pub email: Option<String>,
    pub grupo_venda: String,
    pub ativo: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

// ===== REQUESTS E RESPONSES =====

#[derive(Debug, Deserialize)]
pub struct CriarPedidoRequest {
    pub codigo_cliente: String,
    pub loja_cliente: String,
    pub emissao: String,
    pub natureza: Option<String>, // Default: "10212"
    pub mensagem: Option<String>,
    pub regra_condicao_pagamento_id: i32,
    pub regra_frete_id: i32,
    pub items: Vec<ItemPedidoRequest>,
}

#[derive(Debug, Deserialize)]
pub struct ItemPedidoRequest {
    pub produto_id: i32,
    pub quantidade: i32,
}

#[derive(Debug, Serialize)]
pub struct CriarPedidoResponse {
    pub success: bool,
    pub pedido_id: Option<i32>,
    pub numero_pedido: Option<String>,
    pub total: Option<Decimal>,
    pub message: String,
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct ProdutoComPreco {
    pub id: i32,
    pub codigo: String,
    pub descricao: String,
    pub unidade_medida: String,
    pub quantidade_minima_embalagem: i32,
    pub saldo: i32,
    pub preco: Option<Decimal>,
    pub status: bool,
}

// ===== HELPERS =====

impl Default for CriarPedidoRequest {
    fn default() -> Self {
        Self {
            codigo_cliente: String::new(),
            loja_cliente: String::new(),
            emissao: String::new(),
            natureza: Some("10212".to_string()),
            mensagem: None,
            regra_condicao_pagamento_id: 0,
            regra_frete_id: 0,
            items: Vec::new(),
        }
    }
}
