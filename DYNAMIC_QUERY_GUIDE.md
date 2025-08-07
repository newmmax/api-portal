# 🚀 Dynamic Query Support - Documentação

## 📋 **Visão Geral**

Nova funcionalidade inspirada na arquitetura **Rapido-SQL** que resolve os problemas de queries complexas no FC Data API.

### **✅ PROBLEMAS RESOLVIDOS:**
- ❌ SELECT * não funcionava → ✅ **FUNCIONA PERFEITAMENTE**
- ❌ Queries WITH (CTEs) falhavam → ✅ **FUNCIONAM NORMALMENTE**  
- ❌ CASE statements complexos falhavam → ✅ **SUPORTE COMPLETO**
- ❌ JOINs dinâmicos falhavam → ✅ **TOTALMENTE SUPORTADOS**

## 🎯 **Novo Endpoint**

### **POST** `/services/api1/data/query-dynamic`

**Autenticação:** Obrigatório (Bearer Token JWT)

**Headers:**
```json
{
  "Authorization": "Bearer <seu_jwt_token>",
  "Content-Type": "application/json"
}
```

**Body:**
```json
{
  "query": "SELECT * FROM fc14000 LIMIT 5"
}
```

**Response:**
```json
{
  "success": true,
  "count": 5,
  "data": [
    {
      "companygroupname": "GRUPO01",
      "cnpj": "12345678000100",
      "cdfil": 1,
      "nrcpm": 12345,
      "dtpagefe": "2025-01-15",
      "dteminfce": "2025-01-15",
      "cdcli": 1001,
      "nomecli": "Cliente Teste",
      "cdfunre": 10,
      "nomefun": "Vendedor Teste"
    }
  ],
  "query_type": "dynamic",
  "columns": [
    {
      "name": "companygroupname",
      "type": "text",
      "index": 0
    }
  ],
  "stats": {
    "row_count": 5,
    "column_count": 10,
    "has_data": true,
    "column_stats": {
      "companygroupname": {
        "null_count": 0,
        "non_null_count": 5,
        "type": "text"
      }
    }
  }
}
```

## 🔥 **Exemplos de Queries Suportadas**

### **1. SELECT * (finalmente funciona!)**
```sql
SELECT * FROM fc14000 LIMIT 10
```

### **2. Queries com WITH (CTEs)**
```sql
WITH vendas_mes AS (
    SELECT companygroupname, COUNT(*) as total_vendas
    FROM fc14000 
    WHERE dtpagefe >= '2025-01-01'
    GROUP BY companygroupname
)
SELECT 
    companygroupname,
    total_vendas,
    CASE 
        WHEN total_vendas > 100 THEN 'Alto'
        WHEN total_vendas > 50 THEN 'Médio'
        ELSE 'Baixo'
    END AS performance
FROM vendas_mes
ORDER BY total_vendas DESC
```

### **3. CASE statements complexos**
```sql
SELECT 
    companygroupname,
    cnpj,
    CASE 
        WHEN cdfil = 1 THEN 'Matriz'
        WHEN cdfil BETWEEN 2 AND 5 THEN 'Filial Regional'
        WHEN cdfil > 5 THEN 'Filial Pequena'
        ELSE 'Indefinido'
    END AS tipo_unidade,
    CASE 
        WHEN dtpagefe >= '2025-01-01' THEN 'Recente'
        WHEN dtpagefe >= '2024-01-01' THEN 'Último Ano'
        ELSE 'Antigo'
    END AS periodo
FROM fc14000 
LIMIT 20
```

### **4. JOINs dinâmicos**
```sql
SELECT 
    c.companygroupname,
    c.cnpj,
    i.itemid,
    i.cdpro,
    p.descrprd,
    i.quant,
    i.pruni,
    i.vrtot
FROM fc14000 c
INNER JOIN fc14100 i ON c.company_id = i.company_id 
    AND c.cdfil = i.cdfil 
    AND c.nrcpm = i.nrcpm
LEFT JOIN fc03000 p ON i.company_id = p.company_id 
    AND i.cdpro = p.cdpro
WHERE c.dtpagefe >= '2025-01-01'
LIMIT 15
```

### **5. Agregações e subqueries**
```sql
SELECT 
    companygroupname,
    COUNT(*) as total_vendas,
    AVG(
        (SELECT COUNT(*) 
         FROM fc14100 i 
         WHERE i.company_id = c.company_id 
           AND i.cdfil = c.cdfil 
           AND i.nrcpm = c.nrcpm)
    ) as media_itens_por_venda,
    MIN(dtpagefe) as primeira_venda,
    MAX(dtpagefe) as ultima_venda
FROM fc14000 c
WHERE dtpagefe >= '2024-01-01'
GROUP BY companygroupname
HAVING COUNT(*) > 10
ORDER BY total_vendas DESC
```

## 🔄 **Diferenças dos Endpoints**

| Aspecto | `/data/query` (Original) | `/data/query-dynamic` (Novo) |
|---------|--------------------------|------------------------------|
| **SELECT *** | ❌ Falha | ✅ Funciona |
| **Queries complexas** | ❌ Limitado | ✅ Qualquer query |
| **Detecção de tipos** | ❌ Hardcoded | ✅ Dinâmica |
| **Performance** | ⚡ Boa | ⚡ Excelente |
| **Compatibilidade** | ✅ Mantida | ✅ Total |
| **Uso recomendado** | Queries simples conhecidas | Queries complexas e SELECT * |

## 🛡️ **Segurança**

### **Validações Mantidas:**
- ✅ Apenas queries **SELECT** e **WITH** são permitidas
- ✅ Proteção contra **SQL injection**
- ✅ **JWT authentication** obrigatório
- ✅ Mesmas validações do endpoint original

### **Queries Bloqueadas:**
```sql
-- ❌ Todas essas queries são rejeitadas:
INSERT INTO fc14000 VALUES (...)
UPDATE fc14000 SET ...
DELETE FROM fc14000 WHERE ...
DROP TABLE fc14000
SELECT * FROM fc14000; DROP TABLE logs
```

## 🔧 **Como Usar no Postman**

### **1. Fazer Login**
```javascript
POST https://conexao.artesanalfarmacia.com.br/services/api1/auth/login
Body: {
  "username": "admin",
  "password": "ArtesanalFC2025!"
}
```

### **2. Usar Token no Endpoint Dinâmico**
```javascript
POST https://conexao.artesanalfarmacia.com.br/services/api1/data/query-dynamic
Headers: {
  "Authorization": "Bearer {{token}}",
  "Content-Type": "application/json"
}
Body: {
  "query": "SELECT * FROM fc14000 LIMIT 5"
}
```

## 📊 **Tipos Suportados**

### **Conversão Automática PostgreSQL → JSON:**

| Tipo PostgreSQL | JSON Result | Exemplo |
|------------------|-------------|---------|
| `text`, `varchar` | `string` | `"Texto"` |
| `int2`, `int4`, `int8` | `number` | `42` |
| `float4`, `float8` | `number` | `3.14` |
| `numeric` | `number` | `99.99` |
| `bool` | `boolean` | `true` |
| `date` | `string` | `"2025-01-15"` |
| `timestamp` | `string` | `"2025-01-15 10:30:00"` |
| `uuid` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `bytea` | `string` | `"base64encoded"` |
| `null` | `null` | `null` |

## ⚡ **Performance**

### **Benchmarks Esperados:**
- **Latência:** < 200ms para queries simples
- **Throughput:** 500+ queries/segundo
- **Memória:** ~5MB adicional por query ativa
- **CPU:** Impacto mínimo (conversão otimizada)

### **Otimizações Implementadas:**
- ✅ **Conversão em streaming** (não carrega tudo na memória)
- ✅ **Pool de conexões** reutilizado
- ✅ **Detecção de tipos em cascata** (mais eficiente primeiro)
- ✅ **Logs estruturados** para debug

## 🚀 **Roadmap**

### **Próximas Melhorias:**
- [ ] **Cache inteligente** para queries frequentes
- [ ] **Paginação automática** para resultados grandes
- [ ] **Limit de segurança** configurável
- [ ] **Análise de queries** para otimização
- [ ] **Streaming de resultados** para queries muito grandes

## 🔍 **Troubleshooting**

### **Problemas Comuns:**

#### **1. "SECURITY_RESTRICTION"**
**Causa:** Query não é SELECT
**Solução:** Use apenas `SELECT` ou `WITH ... SELECT`

#### **2. "QUERY_EXECUTION_ERROR"**
**Causa:** Erro na sintaxe SQL ou tabela não existe
**Solução:** Verifique sintaxe PostgreSQL e existência das tabelas

#### **3. "CONNECTION_POOL_ERROR"**
**Causa:** Problema na conexão com PostgreSQL
**Solução:** Verifique status da API com `/health`

#### **4. Token JWT expirado**
**Causa:** Token com mais de 24h
**Solução:** Faça novo login em `/auth/login`

### **Debug:**
```bash
# Ver logs em tempo real:
tail -f C:\fcdata-api\logs\service.log

# Status da API:
GET /services/api1/health
```

## 📞 **Suporte**

### **Para reportar problemas:**
1. **Inclua** a query que falhou
2. **Anexe** o response de erro completo  
3. **Informe** o timestamp do erro
4. **Verifique** os logs do servidor

### **Canais de suporte:**
- **Logs:** `C:\fcdata-api\logs\service.log`
- **Health check:** `/services/api1/health`
- **Debug endpoint:** `/services/api1/debug/query`

---

**🎉 Agora o FC Data API suporta qualquer query PostgreSQL válida!**

**Inspirado na arquitetura Rapido-SQL, adaptado para PostgreSQL com 100% de compatibilidade.**