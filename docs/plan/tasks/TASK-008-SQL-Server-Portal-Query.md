# 🎯 TASK-008: SQL Server Portal - Dynamic Query Support

## 📋 **Informações da Tarefa**
- **ID**: task-008-sql-server-portal-query
- **Prioridade**: 🔴 ALTA
- **Prazo**: 08/08/2025
- **Estimativa**: 3 horas
- **Status**: ⏳ **PENDENTE**
- **Categoria**: Backend
- **Dependência**: TASK-007 (Dynamic Query PostgreSQL) ✅ CONCLUÍDA

## 🎯 **Objetivo**
Implementar suporte a queries dinâmicas para o SQL Server Portal, resolvendo as limitações de SELECT *, CTEs, CASE statements e JOINs no endpoint `/portal/query`.

**RESULTADO**: Portal de Pedidos executando qualquer query SQL Server válida com conversão dinâmica de tipos.

## 📋 **Checklist de Implementação**

### **FASE 1: Análise e Preparação** ⏳
- [ ] 🔍 **Analisar Rapido-SQL crate**: Estudar arquitetura de conversão SQL Server → JSON
- [ ] 🔧 **Identificar componentes reutilizáveis**: Extrair lógica de `query.rs` e `json_converter.rs`
- [ ] 📊 **Mapear pool existente**: Entender bb8-tiberius vs conexão direta Rapido-SQL
- [ ] 🧪 **Validação Rapido-SQL**: Executar exemplos para entender comportamento

### **FASE 2: Implementação Core** ⏳
- [ ] 🎯 **Criar dynamic_portal_handler.rs**: Handler especializado para SQL Server Portal
- [ ] 🔄 **Adaptar extract_value_static()**: Converter função Rapido-SQL para pool bb8-tiberius
- [ ] 📊 **Implementar sql_server_row_to_json()**: Conversão dinâmica Row → JSON
- [ ] 🛡️ **Migrar validações de segurança**: Manter is_select_query() e autenticação JWT

### **FASE 3: Integração Sistema** ⏳
- [ ] 🛠️ **Atualizar src/main.rs**: Adicionar rota `/portal/query-dynamic`
- [ ] 📦 **Atualizar mod.rs**: Incluir novo módulo
- [ ] 🔗 **Integrar pool existente**: Usar `pools.sqlserver_portal.get().await`
- [ ] 📝 **Logs estruturados**: Manter padrão de logging do sistema

## 🔧 **Especificações Técnicas**
### Estrutura de Arquivos
```
src/handlers/
├── dynamic_query_handler.rs (PostgreSQL) ✅
├── portal_dynamic_handler.rs (NOVO - SQL Server Portal)
└── mod.rs (atualizar)
```

### Interfaces Necessárias
```rust
// Estrutura compatível com sistema existente
#[derive(Debug, Deserialize)]
pub struct PortalDynamicQueryRequest {
    pub query: String,
    pub database: Option<String>,
}

// Função principal
pub async fn execute_portal_dynamic_query(
    pools: web::Data<DatabasePools>,
    query_req: web::Json<PortalDynamicQueryRequest>,
) -> Result<HttpResponse>

// Conversor SQL Server específico
fn dynamic_sqlserver_to_json(rows: Vec<tiberius::Row>) -> Value
fn extract_sqlserver_value(row: &tiberius::Row, col_name: &str) -> Value
```

### Endpoint Alvo
```yaml
POST /services/api1/portal/query-dynamic
Authorization: Bearer JWT
Content-Type: application/json
Body: {"query": "SELECT * FROM dbo.clientes LIMIT 5"}

Response: {
  "success": true,
  "count": 5,
  "data": [...],
  "query_type": "dynamic_sqlserver",
  "stats": {...}
}
```

## 🎯 **Critérios de Sucesso**
- [ ] **SELECT * funciona**: `SELECT * FROM dbo.clientes` retorna dados
- [ ] **CTEs funcionam**: `WITH cte AS (...) SELECT * FROM cte` executa
- [ ] **CASE statements**: `SELECT CASE WHEN ... END` funciona
- [ ] **JOINs dinâmicos**: `SELECT * FROM dbo.clientes c JOIN dbo.pedidos p ON ...`
- [ ] **Performance aceitável**: Latência < 300ms para queries simples
- [ ] **Compatibilidade total**: Endpoint original `/portal/query` não afetado

## 📁 **Referências**
### Contextos Relacionados
- `docs/context/CTX-Rapido-SQL.md` - Arquitetura de referência
- `docs/context/CTX-Dynamic-Query-Support.md` - Implementação PostgreSQL

### ADRs Relacionados
- `docs/adrs/ADR-002-Arquitetura-Rapido-SQL.md` - Decisão arquitetural

### Código de Referência
```yaml
Rapido-SQL:
  - D:\PROJETOS\RAPIDO\rapido-sql\src\query.rs
  - D:\PROJETOS\RAPIDO\rapido-sql\src\json_converter.rs
  - D:\PROJETOS\RAPIDO\rapido-sql\examples\practical_usage.rs

FC Data API:
  - src\handlers\dynamic_query_handler.rs (PostgreSQL)
  - src\handlers\portal_handlers.rs (atual limitado)
```

## 📋 **Log de Execução**
| Data/Hora | Ação | Status | Observações |
|-----------|------|--------|-------------|
| 08/08/2025 16:00 | Task criada | 📋 | Especificações definidas, PostgreSQL implementado |

## 🚀 **Estratégia de Implementação**

### **Abordagem Recomendada: Adaptação Híbrida**
1. **Extrair** lógica core da Rapido-SQL (60%)
2. **Adaptar** para pool bb8-tiberius do FC Data API (30%)
3. **Integrar** validações e logs existentes (10%)

### **Vantagens desta Abordagem**
- ✅ Reutiliza código testado da Rapido-SQL
- ✅ Mantém integração com infraestrutura existente
- ✅ Zero risco para funcionalidades atuais
- ✅ Performance otimizada para ambiente FC Data API

### **Pontos de Atenção**
- 🟡 Adaptar tipos Tiberius Row para pool bb8
- 🟡 Manter validações de segurança rigorosas
- 🟡 Logs consistentes com padrão existente
- 🟡 Tratamento de erro robusto

## 🧪 **Plano de Testes**
### Testes Funcionais
```sql
-- Teste 1: SELECT básico
SELECT * FROM dbo.clientes LIMIT 3

-- Teste 2: CTE
WITH top_clientes AS (
    SELECT codigo, razao_social FROM dbo.clientes LIMIT 5
) SELECT * FROM top_clientes

-- Teste 3: CASE complex
SELECT codigo, razao_social,
    CASE 
        WHEN ativo = 1 THEN 'Ativo'
        ELSE 'Inativo'
    END AS status
FROM dbo.clientes

-- Teste 4: JOIN dinâmico
SELECT c.*, p.id as pedido_id
FROM dbo.clientes c
LEFT JOIN dbo.pedidos p ON c.codigo = p.codigo_cliente
```

### Testes de Segurança
```sql
-- Deve bloquear:
INSERT INTO dbo.clientes VALUES (...)
UPDATE dbo.clientes SET ...
DELETE FROM dbo.clientes
DROP TABLE dbo.clientes
```

## 🔄 **Próximas Tarefas Dependentes**
- **TASK-009**: SQL Server Protheus Dynamic Query (similar)
- **TASK-010**: Consolidação e Otimização de Performance
- **TASK-011**: Documentação Unificada dos Endpoints Dinâmicos

---
📅 **Criado**: 08/08/2025  
⏭️ **Próxima**: TASK-009 (Protheus Dynamic Query)  
🎯 **Resultado**: Portal com flexibilidade total de queries SQL Server
