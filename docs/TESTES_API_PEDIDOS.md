# 🧪 TESTES DA API - CRIAÇÃO DE PEDIDOS

## 📋 Pré-requisitos para Testes

### 1. Obter Token JWT
```bash
curl -X POST http://127.0.0.1:8089/services/api1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "ArtesanalFC2025!"
  }'
```

### 2. Buscar Cliente de Teste
```sql
-- Execute no SQL Server para encontrar clientes válidos
SELECT TOP 5 
    id, codigo, loja, razao_social, cnpj, grupo_venda 
FROM sys_pedidos_teste.dbo.clientes 
WHERE ativo = 1 
  AND grupo_venda IS NOT NULL
ORDER BY id
```

### 3. Buscar Produtos com Preço
```sql
-- Produtos com preço para um grupo específico
SELECT TOP 10 
    p.id, p.codigo, p.descricao, p.saldo, 
    p.quantidade_minima_embalagem, pp.preco
FROM sys_pedidos_teste.dbo.produtos p
INNER JOIN sys_pedidos_teste.dbo.precos_produtos pp 
    ON p.codigo = pp.codigo_produto
WHERE p.status = 1 
  AND p.saldo > 0
  AND pp.grupo_venda = 'GRUPO_DO_CLIENTE_AQUI'
ORDER BY p.id
```

### 4. Buscar Regras de Frete e Parcelamento
```sql
-- Regras de frete
SELECT id, descricao, valor_minimo 
FROM sys_pedidos_teste.dbo.regra_frete 
ORDER BY valor_minimo

-- Regras de parcelamento
SELECT id, descricao, valor_minimo, valor_maximo 
FROM sys_pedidos_teste.dbo.regras_parcelamento 
ORDER BY valor_minimo
```

## 🚀 Exemplo de Criação de Pedido

### Request
```bash
curl -X POST http://127.0.0.1:8089/services/api1/portal/pedidos \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer SEU_TOKEN_JWT_AQUI" \
  -d '{
    "codigo_cliente": "000005",
    "loja_cliente": "01",
    "emissao": "2025-01-11",
    "natureza": "10212",
    "mensagem": "Pedido de teste via API",
    "regra_condicao_pagamento_id": 1,
    "regra_frete_id": 1,
    "items": [
      {
        "produto_id": 123,
        "quantidade": 10
      },
      {
        "produto_id": 456,
        "quantidade": 5
      }
    ]
  }'
```

### Response Sucesso
```json
{
  "success": true,
  "pedido_id": 12345,
  "numero_pedido": null,
  "total": "150.50",
  "message": "Pedido criado com sucesso! Confirme para finalizar.",
  "errors": null
}
```

### Response Erro
```json
{
  "success": false,
  "message": "Produto 123 está inativo"
}
```

## ✅ Confirmar Pedido

### Request
```bash
curl -X POST http://127.0.0.1:8089/services/api1/portal/pedidos/12345/confirmar \
  -H "Authorization: Bearer SEU_TOKEN_JWT_AQUI"
```

### Response
```json
{
  "success": true,
  "message": "Pedido confirmado com sucesso!"
}
```

## 🔍 Listar Produtos Disponíveis

### Request
```bash
curl -X GET "http://127.0.0.1:8089/services/api1/portal/produtos?cliente_id=5&limite=10" \
  -H "Authorization: Bearer SEU_TOKEN_JWT_AQUI"
```

### Response
```json
{
  "success": true,
  "data": [
    {
      "id": 123,
      "codigo": "PROD001",
      "descricao": "Produto Teste",
      "unidade_medida": "UN",
      "quantidade_minima_embalagem": 1,
      "saldo": 100,
      "preco": "15.50",
      "status": true,
      "grupo_venda": "GRUPO1"
    }
  ],
  "count": 1
}
```

## ⚠️ Erros Comuns

### 1. Cliente não encontrado
- Verifique se o código e loja existem
- Confirme que o cliente está ativo

### 2. Produto sem preço
- O produto precisa ter preço para o grupo_venda do cliente
- Verifique na tabela precos_produtos

### 3. Valor mínimo de frete
- O total do pedido deve atingir o valor_minimo da regra de frete escolhida

### 4. Produto sem saldo
- Verifique o campo saldo do produto
- Produto precisa ter saldo >= quantidade solicitada

## 📝 Notas Importantes

1. **Base de Teste**: Estamos usando `sys_pedidos_teste` e `sigaofc_teste`
2. **Status Inicial**: Pedidos são criados com status "a confirmar"
3. **Confirmação**: Ao confirmar, a data de emissão é atualizada para hoje
4. **Transações**: Criação de pedido usa transação (rollback em caso de erro)
5. **Validações**: Todas as regras de negócio do Portal são respeitadas

## 📝 Buscar Pedido Específico

### Request
```bash
curl -X GET http://127.0.0.1:8089/services/api1/portal/pedidos/12345 \
  -H "Authorization: Bearer SEU_TOKEN_JWT_AQUI"
```

### Response
```json
{
  "success": true,
  "data": {
    "id": 12345,
    "cliente_id": 5,
    "codigo_cliente": "000005",
    "numero_pedido": null,
    "loja_cliente": "01",
    "emissao": "2025-01-11",
    "mensagem": "Pedido de teste",
    "natureza": "10212",
    "status_pedido": "a confirmar",
    "regra_condicao_pagamento_id": 1,
    "regra_frete_id": 1,
    "razao_social": "Cliente Teste",
    "grupo_venda": "GRUPO1",
    "items": [
      {
        "id": 1,
        "produto_id": 123,
        "quantidade": 10,
        "preco_unitario": 15.50,
        "codigo_produto": "PROD001",
        "descricao_produto": "Produto Teste",
        "unidade_medida": "UN"
      }
    ]
  }
}
```

## ✏️ Atualizar Pedido

### Request
```bash
curl -X PUT http://127.0.0.1:8089/services/api1/portal/pedidos/12345 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer SEU_TOKEN_JWT_AQUI" \
  -d '{
    "codigo_cliente": "000005",
    "loja_cliente": "01",
    "emissao": "2025-01-11",
    "natureza": "10212",
    "mensagem": "Pedido atualizado via API",
    "regra_condicao_pagamento_id": 2,
    "regra_frete_id": 1,
    "items": [
      {
        "produto_id": 123,
        "quantidade": 20
      },
      {
        "produto_id": 789,
        "quantidade": 3
      }
    ]
  }'
```

### Response
```json
{
  "success": true,
  "message": "Pedido atualizado com sucesso!",
  "pedido_id": 12345,
  "total": "350.00"
}
```

### Regras de Atualização
- ⚠️ Só pode atualizar pedidos com status "a confirmar"
- ⚠️ Pedidos confirmados ou integrados NÃO podem ser editados
- Items antigos são deletados e novos são inseridos
- Todas as validações de criação se aplicam

## 🗑️ Deletar Pedido

### Request
```bash
curl -X DELETE http://127.0.0.1:8089/services/api1/portal/pedidos/12345 \
  -H "Authorization: Bearer SEU_TOKEN_JWT_AQUI"
```

### Response
```json
{
  "success": true,
  "message": "Pedido excluído com sucesso!"
}
```

### Regras de Exclusão
- ⚠️ Só pode deletar pedidos com status "a confirmar"
- ⚠️ Pedidos confirmados ou integrados NÃO podem ser excluídos
- Exclusão em cascata (deleta automaticamente os items)

## 📊 Resumo de Operações por Status

| Operação | Status "a confirmar" | Status "confirmado" | Status "integrado" |
|----------|---------------------|--------------------|--------------------|
| Visualizar | ✅ Permitido | ✅ Permitido | ✅ Permitido |
| Editar | ✅ Permitido | ❌ Bloqueado | ❌ Bloqueado |
| Deletar | ✅ Permitido | ❌ Bloqueado | ❌ Bloqueado |
| Confirmar | ✅ Permitido | ❌ Já confirmado | ❌ Bloqueado |

## 🔄 Fluxo Completo de Teste

```bash
# 1. Login para obter token
TOKEN=$(curl -s -X POST http://127.0.0.1:8089/services/api1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "ArtesanalFC2025!"}' \
  | jq -r '.token')

# 2. Criar pedido
PEDIDO_ID=$(curl -s -X POST http://127.0.0.1:8089/services/api1/portal/pedidos \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{...}' | jq -r '.pedido_id')

# 3. Buscar pedido criado
curl -X GET http://127.0.0.1:8089/services/api1/portal/pedidos/$PEDIDO_ID \
  -H "Authorization: Bearer $TOKEN"

# 4. Atualizar pedido (só funciona se status = "a confirmar")
curl -X PUT http://127.0.0.1:8089/services/api1/portal/pedidos/$PEDIDO_ID \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{...}'

# 5. Confirmar pedido
curl -X POST http://127.0.0.1:8089/services/api1/portal/pedidos/$PEDIDO_ID/confirmar \
  -H "Authorization: Bearer $TOKEN"

# 6. Tentar editar após confirmado (vai falhar)
curl -X PUT http://127.0.0.1:8089/services/api1/portal/pedidos/$PEDIDO_ID \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{...}'

# 7. Para deletar, precisa estar com status "a confirmar"
```