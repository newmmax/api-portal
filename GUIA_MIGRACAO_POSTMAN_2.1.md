# 🚀 Guia de Migração Postman - Dynamic Query Support

## 📋 **Visão Geral**

Este guia explica como usar as novas collections e environments da FC Data API 2.1 com suporte completo ao Dynamic Query.

## 📦 **Arquivos Atualizados**

### **📋 Collections:**
| Arquivo | Versão | Descrição | Uso |
|---------|--------|-----------|-----|
| `FC_Data_API_2.0.postman_collection.json` | 2.0.0 | Collection original | Produção estável |
| `FC_Data_API_2.1_DYNAMIC.postman_collection.json` | **2.1.0** | **Collection com Dynamic Query** | **Uso recomendado** |

### **🌐 Environments:**
| Arquivo | Versão | Descrição | Uso |
|---------|--------|-----------|-----|
| `FC_Data_API_Dev.postman_environment.json` | 2.0.0 | Environment dev original | Compatibilidade |
| `FC_Data_API_Prod.postman_environment.json` | 2.0.0 | Environment prod original | Compatibilidade |
| `FC_Data_API_2.1_Dev_DYNAMIC.postman_environment.json` | **2.1.0** | **Dev + Dynamic Query** | **Desenvolvimento** |
| `FC_Data_API_2.1_Prod_DYNAMIC.postman_environment.json` | **2.1.0** | **Prod + Dynamic Query** | **Produção** |

## 🚀 **Como Migrar**

### **Passo 1: Importar Nova Collection**
1. Abrir Postman
2. **Import** → **Upload Files**
3. Selecionar: `FC_Data_API_2.1_DYNAMIC.postman_collection.json`
4. ✅ Collection "FC Data API 2.1 - Sistema Completo + Dynamic Query" aparecerá

### **Passo 2: Importar Environments Atualizados**
1. **Import** → **Upload Files**
2. Selecionar os environments 2.1:
   - `FC_Data_API_2.1_Dev_DYNAMIC.postman_environment.json`
   - `FC_Data_API_2.1_Prod_DYNAMIC.postman_environment.json`

### **Passo 3: Configurar Environment Ativo**
1. **Top-right dropdown** → Selecionar environment:
   - **Desenvolvimento**: "FC Data API 2.1 - Desenvolvimento + Dynamic Query"
   - **Produção**: "FC Data API 2.1 - Produção + Dynamic Query"

## 🎯 **Nova Seção: Query Dinâmica**

### **📍 Localização na Collection:**
```
FC Data API 2.1 - Sistema Completo + Dynamic Query
├── 🔐 Autenticação
├── 📊 Data FC (PostgreSQL)
│   ├── Vendas - Query Principal  
│   ├── Vendas Detalhadas
│   ├── Query Customizada
│   └── 🚀 Query Dinâmica (NEW!) ← AQUI!
│       ├── SELECT * Básico
│       ├── WITH (CTE) Complexa  
│       ├── JOIN Dinâmico Multi-Tabela
│       ├── Agregações e Subqueries
│       └── Teste de Tipos Diversos
```

## 🧪 **Como Testar**

### **1. Fazer Login (obrigatório):**
```
POST {{base_url}}/auth/login
Body: {
  "username": "admin", 
  "password": "ArtesanalFC2025!"
}
```
**⚠️ Importante:** Token é salvo automaticamente nas variáveis!

### **2. Testar SELECT * (finalmente funciona!):**
```
📍 Query Dinâmica → SELECT * Básico
POST {{base_url}}/data/query-dynamic
Body: {
  "query": "SELECT * FROM fc14000 LIMIT 5"
}
```

### **3. Testar CTE Complexa:**
```
📍 Query Dinâmica → WITH (CTE) Complexa  
POST {{base_url}}/data/query-dynamic
Body: { "query": "WITH vendas_mes AS (...) SELECT ..." }
```

### **4. Testar JOIN Dinâmico:**
```
📍 Query Dinâmica → JOIN Dinâmico Multi-Tabela
POST {{base_url}}/data/query-dynamic
Body: { "query": "SELECT c.*, i.*, p.descrprd FROM fc14000 c..." }
```

## ✅ **Validações Automáticas**

### **Testes Incluídos na Collection:**
- ✅ **Status Code 200**
- ✅ **Response Structure** (success, data, query_type)
- ✅ **Dynamic Type** (query_type === "dynamic")
- ✅ **Columns Metadata** (presença de columns e stats)
- ✅ **Data Validation** (array de dados)

### **Como Ver Resultados dos Testes:**
1. Executar request
2. **Test Results tab** → Ver validações automáticas
3. **Console** → Ver logs informativos

## 🎯 **Variáveis Úteis**

### **Novas Variáveis nos Environments 2.1:**
```yaml
{{dynamic_query_endpoint}}: 
  - "{{base_url}}/data/query-dynamic"

{{sample_select_all}}:
  - "SELECT * FROM fc14000 LIMIT 5"

{{sample_cte_query}}:
  - "WITH vendas AS (...) SELECT * FROM vendas LIMIT 3"
```

### **Como Usar as Variáveis:**
```json
{
  "query": "{{sample_select_all}}"
}
```

## 🔄 **Comparação de Resultados**

### **Endpoint Original vs Dinâmico:**

**❌ Original (falha com SELECT *):**
```
POST /data/query
{"query": "SELECT * FROM fc14000 LIMIT 3"}
→ ERRO: Tipos não mapeados
```

**✅ Dinâmico (funciona perfeitamente):**
```
POST /data/query-dynamic  
{"query": "SELECT * FROM fc14000 LIMIT 3"}
→ SUCCESS: Detecção automática de tipos
```

### **Estrutura de Resposta Dinâmica:**
```json
{
  "success": true,
  "count": 3,
  "data": [...],
  "query_type": "dynamic",    // ← Identificador
  "columns": [...],           // ← Metadados das colunas
  "stats": {                  // ← Estatísticas avançadas
    "row_count": 3,
    "column_count": 10,
    "has_data": true,
    "column_stats": {...}
  }
}
```

## 🛡️ **Segurança**

### **Validações Mantidas:**
- ✅ **JWT obrigatório** em todos os endpoints dinâmicos
- ✅ **Apenas SELECT e WITH** permitidos
- ✅ **Proteção SQL injection** mantida
- ✅ **Logs de auditoria** para todas as queries

### **Queries Bloqueadas (mesmo comportamento):**
```sql
-- ❌ Estas queries falham em ambos endpoints:
INSERT INTO fc14000 VALUES (...)
UPDATE fc14000 SET ...  
DELETE FROM fc14000
DROP TABLE fc14000
```

## 📊 **Performance**

### **Benchmarks Esperados:**
| Tipo Query | Original | Dinâmico | Diferença |
|------------|----------|----------|-----------|
| SELECT simples | ~100ms | ~120ms | +20% |
| **SELECT *** | ❌ FALHA | ~150ms | ✅ **FUNCIONA** |
| **WITH (CTE)** | ❌ FALHA | ~200ms | ✅ **FUNCIONA** |  
| **CASE complex** | ❌ FALHA | ~180ms | ✅ **FUNCIONA** |

**💡 Trade-off:** Ligeiro overhead em troca de funcionalidade completa.

## 🎛️ **Configurações Avançadas**

### **Timeout para Queries Grandes:**
Se necessário, ajustar timeout no Postman:
1. **Settings** → **General** → **Request timeout**
2. Aumentar para **30 segundos** (queries complexas)

### **Logs Detalhados:**
No **Console** (View → Show Postman Console):
```javascript
🚀 Executando query dinâmica...
💡 Esta funcionalidade resolve os problemas de SELECT * e queries complexas!
Token JWT salvo: eyJ0eXAiOiJKV1...
```

## 🔄 **Migração Gradual**

### **Estratégia Recomendada:**
1. **Manter** collection 2.0 como backup
2. **Usar** collection 2.1 para novos testes
3. **Migrar** gradualmente queries complexas
4. **Validar** performance em cada query

### **Rollback (se necessário):**
1. **Voltar** para collection 2.0
2. **Usar** environments originais
3. **Reportar** problemas encontrados

## 📞 **Suporte**

### **Para Problemas:**
1. **Verificar** se token JWT está válido (24h)
2. **Conferir** environment selecionado
3. **Consultar** console para logs de erro
4. **Verificar** sintaxe SQL PostgreSQL

### **Resources de Debug:**
- **Health Check**: `GET {{base_url}}/health`
- **Token Validation**: `GET {{base_url}}/auth/validate`
- **Query Debug**: `GET {{base_url}}/debug/query`

---

## 🎉 **Conclusão**

**A collection 2.1 resolve DEFINITIVAMENTE os problemas de SELECT *, CTEs e queries complexas!**

### **✅ Agora Funciona:**
- SELECT * de qualquer tabela
- WITH (CTEs) de qualquer complexidade
- CASE statements aninhados
- JOINs dinâmicos multi-tabela
- Detecção automática de tipos PostgreSQL

### **🚀 Próximos Passos:**
1. Importar collections e environments 2.1
2. Testar SELECT * básico
3. Experimentar CTEs complexas
4. Explorar análises antes impossíveis

**Bem-vindo ao futuro das queries dinâmicas no FC Data API! 🚀**