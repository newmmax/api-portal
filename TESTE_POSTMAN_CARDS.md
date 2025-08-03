# 🧪 TESTE POSTMAN - Endpoints Cards Trello

## 📋 **PRÉ-REQUISITOS**

### **1. Obter Token JWT**
```bash
POST https://conexao.artesanalfarmacia.com.br/services/api1/auth/login
Content-Type: application/json

{
  "username": "admin_prod",
  "password": "Pr0duc@0_FC_2025!Art3s@n@l"
}
```

**Resposta esperada:**
```json
{
  "success": true,
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "expires_in": 86400
}
```

---

## 🎯 **TESTES DOS ENDPOINTS**

### **Card 01: Recompra Inteligente**

#### **Teste Básico - Franquia Palmas**
```bash
GET https://conexao.artesanalfarmacia.com.br/services/api1/analytics/recompra-inteligente?cnpj=17.311.174/0001-78
Authorization: Bearer {seu_jwt_token}
```

#### **Teste com Parâmetros Customizados**
```bash
GET https://conexao.artesanalfarmacia.com.br/services/api1/analytics/recompra-inteligente?cnpj=17.311.174/0001-78&periodo_dias=180&limite=20
Authorization: Bearer {seu_jwt_token}
```

#### **Teste - Franquia Inhumas**
```bash
GET https://conexao.artesanalfarmacia.com.br/services/api1/analytics/recompra-inteligente?cnpj=18.803.142/0001-52
Authorization: Bearer {seu_jwt_token}
```

---

### **Card 02: Oportunidades na Rede**

#### **Teste Básico - Franquia Palmas**
```bash
GET https://conexao.artesanalfarmacia.com.br/services/api1/analytics/oportunidades-rede?cnpj=17.311.174/0001-78
Authorization: Bearer {seu_jwt_token}
```

#### **Teste com Parâmetros Customizados**
```bash
GET https://conexao.artesanalfarmacia.com.br/services/api1/analytics/oportunidades-rede?cnpj=17.311.174/0001-78&periodo_dias=60&limite=30
Authorization: Bearer {seu_jwt_token}
```

#### **Teste - Franquia Itumbiara**
```bash
GET https://conexao.artesanalfarmacia.com.br/services/api1/analytics/oportunidades-rede?cnpj=20.246.626/0001-90
Authorization: Bearer {seu_jwt_token}
```

---

## 📊 **CNPJs DE TESTE (Franqueados Reais)**

### **Top 3 Franqueados para Teste**
```yaml
Franquia_Palmas:
  cnpj: "17.311.174/0001-78"
  cod_totvs: "2" 
  email: "palmas@artesanalfranquia.com"
  grupo_venda: "000005"

Franquia_Inhumas:
  cnpj: "18.803.142/0001-52"
  cod_totvs: "3"
  email: "inhumas@artesanalfranquia.com"
  grupo_venda: "000005"

Franquia_Itumbiara:
  cnpj: "20.246.626/0001-90"
  cod_totvs: "4"
  email: "itumbiara@artesanalfranquia.com"
  grupo_venda: "000005"
```

---

## ✅ **VALIDAÇÕES ESPERADAS**

### **Card 01 - Recompra Inteligente**
```json
{
  "success": true,
  "cnpj": "17.311.174/0001-78",
  "periodo_dias": 90,
  "produtos_recompra": [
    {
      "codigo_produto": "PA000002",
      "descricao_produto": "ARTESANAL COLAGENO ABACAXI/LIMAO 300GR",
      "categoria": "ARTESANAL",
      "frequencia_compra": 8,
      "quantidade_media": 25.5,
      "valor_medio": 1425.50,
      "dias_ultima_compra": 15,
      "score_recompra": 5.33,
      "produtos_relacionados": [...]
    }
  ],
  "total_produtos": 10,
  "algoritmo": "score_baseado_em_frequencia_e_recencia"
}
```

**Validações:**
- ✅ `success: true`
- ✅ `cnpj` correto retornado
- ✅ `produtos_recompra` é array com dados
- ✅ `score_recompra` é número > 0
- ✅ `produtos_relacionados` existe (pode ser vazio)

### **Card 02 - Oportunidades na Rede**
```json
{
  "success": true,
  "cnpj": "17.311.174/0001-78",
  "grupo_abc": "B",
  "periodo_dias": 90,
  "oportunidades": [
    {
      "codigo_produto": "PA000008",
      "descricao_produto": "ARTESANAL FPS45 BEGE FACIAL 60GR",
      "categoria": "ARTESANAL",
      "media_franqueado": 10.0,
      "media_rede": 35.5,
      "diferenca_percentual": -71.8,
      "potencial_adicional": 25.5,
      "grupo_abc": "B",
      "prioridade": "alta"
    }
  ],
  "total_oportunidades": 15,
  "algoritmo": "comparacao_vs_media_grupo_abc"
}
```

**Validações:**
- ✅ `success: true`
- ✅ `grupo_abc` é "A", "B" ou "C"
- ✅ `oportunidades` é array
- ✅ `diferenca_percentual` é negativo (indicando oportunidade)
- ✅ `prioridade` é "alta", "media" ou "baixa"

---

## 🚨 **POSSÍVEIS ERROS E SOLUÇÕES**

### **Erro 401 - Unauthorized**
```json
{
  "error": "Token JWT inválido ou expirado"
}
```
**Solução**: Obter novo token via `/auth/login`

### **Erro 500 - Database Error** 
```json
{
  "success": false,
  "message": "Erro de conexão com Portal",
  "error": "Connection failed"
}
```
**Solução**: Verificar se FC Data API está rodando e bancos acessíveis

### **Resposta Vazia**
```json
{
  "success": true,
  "produtos_recompra": [],
  "total_produtos": 0
}
```
**Causa**: CNPJ sem histórico de pedidos ou período muito restritivo
**Solução**: Testar com CNPJs conhecidos ou aumentar período

---

## 🔍 **DEBUG - Queries Internas**

### **Verificar Se CNPJ Existe**
```sql
-- Via endpoint /portal/query
SELECT id, nome, cnpj, grupo_venda 
FROM clientes 
WHERE cnpj = '17.311.174/0001-78' 
  AND deleted_at IS NULL
```

### **Verificar Histórico de Pedidos**
```sql
-- Via endpoint /portal/query  
SELECT COUNT(*) as total_pedidos
FROM pedidos p
INNER JOIN clientes c ON p.cliente_id = c.id
WHERE c.cnpj = '17.311.174/0001-78'
  AND p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
  AND p.created_at >= DATEADD(day, -90, GETDATE())
```

### **Verificar Produtos Mais Comprados**
```sql
-- Via endpoint /portal/query
SELECT TOP 5
    i.codigo_produto,
    i.descricao_produto,
    COUNT(*) as total_pedidos
FROM pedidos p
INNER JOIN items i ON p.id = i.pedido_id  
INNER JOIN clientes c ON p.cliente_id = c.id
WHERE c.cnpj = '17.311.174/0001-78'
  AND p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
GROUP BY i.codigo_produto, i.descricao_produto
ORDER BY total_pedidos DESC
```

---

## 📈 **MÉTRICAS DE SUCESSO**

### **Performance Esperada**
- ⏱️ **Latência**: < 3 segundos por endpoint
- 📊 **Dados**: Retorno com pelo menos alguns produtos/oportunidades
- 🔄 **Disponibilidade**: 99% uptime

### **Qualidade dos Dados**
- ✅ **Recompra**: Score correlacionado com frequência real
- ✅ **Oportunidades**: Diferenças significativas vs rede
- ✅ **Cross-selling**: Produtos relacionados lógicos

**RESULTADO ESPERADO**: Endpoints funcionando com dados reais dos 81 franqueados! 🎯
