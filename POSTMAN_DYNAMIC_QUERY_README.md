# 🚀 FC Data API 2.1 - Dynamic Query Collection

## 📋 **Novos Endpoints Adicionados**

### **🚀 Query Dinâmica (NOVO!)**
Seção completamente nova na collection com 5 exemplos práticos:

#### **1. SELECT * Básico**
- **Endpoint:** `POST /data/query-dynamic`
- **Funcionalidade:** Finalmente SELECT * funciona!
- **Exemplo:**
```json
{
  "query": "SELECT * FROM fc14000 LIMIT 5"
}
```

#### **2. WITH (CTE) Complexa**
- **Funcionalidade:** CTEs com CASE statements complexos
- **Exemplo:**
```sql
WITH vendas_mes AS (
    SELECT 
        companygroupname, 
        COUNT(*) as total_vendas,
        AVG(CASE WHEN cdfunre IS NOT NULL THEN 1.0 ELSE 0.0 END) as tem_vendedor
    FROM fc14000 
    WHERE dtpagefe >= '2024-01-01'
    GROUP BY companygroupname
)
SELECT 
    companygroupname,
    total_vendas,
    ROUND(tem_vendedor * 100, 2) as perc_vendedor,
    CASE 
        WHEN total_vendas > 100 THEN 'Alto Volume'
        WHEN total_vendas > 50 THEN 'Médio Volume'
        ELSE 'Baixo Volume'
    END AS classificacao
FROM vendas_mes
ORDER BY total_vendas DESC
LIMIT 10
```

#### **3. JOIN Dinâmico Multi-Tabela**
- **Funcionalidade:** JOINs dinâmicos entre múltiplas tabelas
- **Detecção:** Automática de tipos em todas as colunas

#### **4. Agregações e Subqueries**
- **Funcionalidade:** Queries complexas com subqueries e agregações
- **Analytics:** Análise completa de performance de vendas

#### **5. Teste de Tipos Diversos**
- **Funcionalidade:** Validação da detecção automática de tipos
- **Cobertura:** String, int, decimal, boolean, date, timestamp, null

## 🔧 **Testes Automatizados Incluídos**

### **Validações Automáticas:**
- ✅ Status code 200
- ✅ Estrutura de resposta dinâmica
- ✅ Propriedade `query_type: "dynamic"`
- ✅ Presença de `columns` e `stats`
- ✅ Validação de dados retornados

### **Scripts de Pre-request:**
- 🚀 Logs informativos sobre funcionalidade dinâmica
- 💡 Dicas sobre resolução de problemas

## 📊 **Estrutura da Resposta Dinâmica**

```json
{
  "success": true,
  "count": 5,
  "data": [
    {
      "companygroupname": "GRUPO01",
      "cnpj": "12345678000100",
      // ... todas as colunas detectadas automaticamente
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

## 🎯 **Como Usar**

### **1. Fazer Login:**
```
POST {{base_url}}/auth/login
```

### **2. Usar Token nos Novos Endpoints:**
```
Authorization: Bearer {{token}}
```

### **3. Testar SELECT * (finalmente!):**
```json
POST {{base_url}}/data/query-dynamic
{
  "query": "SELECT * FROM fc14000 LIMIT 3"
}
```

## ⚡ **Performance Esperada**

| Tipo de Query | Endpoint Original | Endpoint Dinâmico | Status |
|---------------|-------------------|-------------------|---------|
| **SELECT simples** | ~100ms | ~120ms | ✅ +20% |
| **SELECT *** | ❌ FALHA | ~150ms | ✅ FUNCIONA |
| **WITH (CTE)** | ❌ FALHA | ~200ms | ✅ FUNCIONA |
| **CASE complex** | ❌ FALHA | ~180ms | ✅ FUNCIONA |

## 🛡️ **Segurança**

### **Validações Mantidas:**
- ✅ Apenas queries SELECT e WITH
- ✅ JWT authentication obrigatório
- ✅ Proteção contra SQL injection
- ✅ Logs detalhados para auditoria

### **Queries Bloqueadas:**
- ❌ INSERT, UPDATE, DELETE
- ❌ DROP, ALTER, TRUNCATE
- ❌ EXEC, EXECUTE
- ❌ Múltiplas statements com `;`

## 📈 **Benefícios**

### **✅ Problemas Resolvidos:**
- **SELECT * finalmente funciona**
- **CTEs complexas suportadas**
- **CASE statements funcionam**
- **JOINs dinâmicos funcionam**
- **Detecção automática de tipos**

### **🚀 Novas Possibilidades:**
- Análises complexas antes impossíveis
- Exploração livre dos dados FC
- Queries ad-hoc para analistas
- Debugging avançado de dados
- Prototipagem rápida de relatórios

## 🔄 **Compatibilidade**

### **100% Compatível:**
- ✅ Todos os endpoints existentes funcionam normalmente
- ✅ Novo endpoint é paralelo (não substitui o original)
- ✅ Mesma autenticação e segurança
- ✅ Rollback simples se necessário

## 📚 **Documentação Completa**

### **Guias Disponíveis:**
- 📖 `DYNAMIC_QUERY_GUIDE.md` - Documentação completa
- 🧪 `TEST_DYNAMIC_QUERY.md` - Guia de testes
- 📋 `FC_Data_API_2.1.postman_collection.json` - Esta collection

---

**🎉 Agora o FC Data API suporta qualquer query PostgreSQL válida!**

**Inspirado na arquitetura Rapido-SQL, adaptado para PostgreSQL com total compatibilidade.**
