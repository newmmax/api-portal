# 📋 REGRAS DE NEGÓCIO - CRIAÇÃO DE PEDIDOS

## 🎯 Visão Geral
O sistema de pedidos do Portal possui diversas regras de validação e cálculo que devem ser respeitadas na API Rust.

## 🔍 REGRAS PRINCIPAIS

### 1. **AUTENTICAÇÃO E CLIENTE**
- ✅ Cliente deve estar autenticado (JWT)
- ✅ Cliente deve estar ativo
- ✅ Cliente possui `grupo_venda` que define preços e regras
- ✅ `tabela_precos` vem de `regraCliente`

### 2. **VALIDAÇÕES DE PRODUTOS**
```rust
// Produto elegível para pedido deve ter:
- status = true (ativo)
- saldo > 0 (tem estoque)
- Preço definido para o grupo_venda do cliente (tabela precos_produtos)
- quantidade >= quantidade_minima_embalagem
```

### 3. **SISTEMA DE PREÇOS**
```sql
-- Preço é determinado por grupo_venda
SELECT preco FROM precos_produtos 
WHERE codigo_produto = ? AND grupo_venda = ?
```
- Cada cliente tem um `grupo_venda`
- Produtos têm preços diferentes por grupo
- Se não houver preço para o grupo, produto não pode ser vendido

### 4. **QUANTIDADE MÍNIMA POR EMBALAGEM**
```rust
// IMPORTANTE: Código está comentado mas regra ainda existe
// quantidade_ajustada = max(ceil(quantidade / quantidade_minima_embalagem), 1) * quantidade_minima_embalagem
// Atualmente usando: quantidade_ajustada = quantidade (direto)
```
⚠️ **NOTA**: Verificar com negócio se regra deve ser reativada

### 5. **REGRAS DE FRETE**
```rust
// Tabela: regra_frete
struct RegraFrete {
    id: i32,
    valor_minimo: f64,
    descricao: String,
    codigo_protheus: String,
}

// Lógica: Aplica-se a regra onde valor_minimo <= total_pedido
// Sistema seleciona automaticamente a regra aplicável
```

### 6. **REGRAS DE PARCELAMENTO**
```rust
// Tabela: regras_parcelamento
struct RegraParcelamento {
    id: i32,
    valor_minimo: f64,
    valor_maximo: Option<f64>,
    codigo_protheus: String,
    descricao: String,
}

// Lógica: Mostra opções onde valor_minimo <= total_pedido <= valor_maximo
// Cliente escolhe entre as opções disponíveis
```

### 7. **CAMPOS OBRIGATÓRIOS DO PEDIDO**
```rust
struct CriarPedidoRequest {
    // Obrigatórios
    codigo_cliente: String,      // Ex: "000005"
    loja_cliente: String,        // Ex: "01"
    emissao: String,            // Data formato YYYY-MM-DD
    natureza: String,           // Default: "10212"
    regra_condicao_pagamento_id: i32,  // ID da regra de parcelamento
    regra_frete_id: i32,        // ID da regra de frete
    
    // Opcional
    mensagem: Option<String>,    // Observações do pedido
    
    // Items
    items: Vec<ItemPedido>,
}

struct ItemPedido {
    produto_id: i32,
    quantidade: i32,        // Deve respeitar quantidade_minima_embalagem
    // preco_unitario é calculado automaticamente pelo grupo_venda
}
```

### 8. **STATUS DO PEDIDO**
```rust
// Fluxo de status:
"a confirmar" -> "confirmado" -> "integrado" -> ["Confirmado ERP", "Em Separação", "Faturado", "Pronto pra Coleta"]

// Pedido sempre criado com status = "a confirmar"
// Cliente precisa confirmar explicitamente
// Ao confirmar: atualiza emissao para data atual
```

### 9. **VALIDAÇÕES DE NEGÓCIO**

#### Na Criação:
1. Cliente autenticado e ativo
2. Produtos com status=true e saldo>0
3. Produtos com preço para o grupo_venda
4. Total do pedido > 0
5. Regra de frete aplicável ao valor
6. Regra de parcelamento válida para o valor

#### Na Confirmação:
1. Pedido deve estar com status "a confirmar"
2. Não pode confirmar pedido já confirmado
3. Atualiza data de emissão para NOW()

### 10. **TRANSAÇÕES E ATOMICIDADE**
```rust
// Usar transação para garantir atomicidade
BEGIN TRANSACTION
    1. Criar cabeçalho do pedido
    2. Para cada item:
        - Validar produto
        - Buscar preço por grupo
        - Criar item com preço correto
    3. Calcular total
    4. Validar regras
COMMIT ou ROLLBACK
```

## 📊 QUERIES IMPORTANTES

### Buscar produtos disponíveis:
```sql
SELECT p.*, pp.preco 
FROM produtos p
INNER JOIN precos_produtos pp ON p.codigo = pp.codigo_produto
WHERE p.status = 1 
  AND p.saldo > 0
  AND pp.grupo_venda = ?
```

### Buscar regras de frete aplicáveis:
```sql
SELECT * FROM regra_frete 
WHERE valor_minimo <= ? 
ORDER BY valor_minimo DESC
```

### Buscar regras de parcelamento:
```sql
SELECT * FROM regras_parcelamento 
WHERE valor_minimo <= ? 
  AND (valor_maximo IS NULL OR valor_maximo >= ?)
ORDER BY valor_minimo
```

## ⚠️ PONTOS DE ATENÇÃO

1. **Quantidade Mínima**: Código comentado mas regra pode voltar
2. **Natureza**: Sempre "10212" por padrão
3. **Preços**: SEMPRE pelo grupo_venda, nunca do produto direto
4. **Validação de Total**: Código comentado que validava total calculado vs informado
5. **Cliente**: Pedido sempre vinculado ao cliente autenticado

## 🧪 DADOS DE TESTE

Use as bases de teste para validar:
- `syspedidos_teste` - Portal teste
- `sigaofc_teste` - Protheus teste

Exemplo de cliente teste:
```sql
SELECT TOP 1 * FROM syspedidos_teste.dbo.clientes 
WHERE ativo = 1 AND grupo_venda IS NOT NULL
```
