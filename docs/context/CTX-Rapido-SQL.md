# 🎯 CTX-Rapido-SQL

## 📋 **O que é**
Crate Rust especializada em executar queries SQL Server e retornar resultados em JSON de forma dinâmica, resolvendo limitações de SELECT * e queries complexas.

## ✅ **Status**
- Status: ✅ FUNCIONAL e TESTADA
- Localização: `D:\PROJETOS\RAPIDO\rapido-sql`
- Última atualização: 08/08/2025
- Uso: Inspiração para FC Data API

## 🚀 **Como usar**
### Como Biblioteca
```rust
use rapido_sql::{RapidoSql, ConnectionConfig};

let config = ConnectionConfig::new("localhost", "database")
    .with_sql_auth("user", "pass");
let mut rapido = RapidoSql::new(config).await?;
let result = rapido.execute_query("SELECT * FROM tabela").await?;
```

### Como Inspiração (FC Data API)
- Estratégia de conversão cascata
- Detecção dinâmica de tipos SQL Server
- Fallback inteligente para tipos desconhecidos

## 🔧 **APIs e Componentes**
### Principais Módulos
- `query.rs`: Execução de queries com QueryExecutor
- `json_converter.rs`: Conversão dinâmica SQL Server → JSON
- `connection.rs`: Gerenciamento de conexões Tiberius
- `error.rs`: Tratamento robusto de erros

### Funcionalidades Chave
- `execute_query()`: Executa qualquer query SQL Server
- `extract_value_static()`: Conversão cascata de tipos
- `process_stream_static()`: Processamento de resultados
- `ResultToJson`: Conversor com estatísticas avançadas

## 💡 **Exemplos Práticos**
### Caso de Uso 1: SELECT * (funciona perfeitamente)
```sql
SELECT * FROM sys.databases
-- ✅ Detecta automaticamente todos os tipos
```

### Caso de Uso 2: CTE Complexa
```sql
WITH NumerosCTE AS (
    SELECT 1 AS numero
    UNION ALL
    SELECT numero + 1 FROM NumerosCTE WHERE numero < 5
)
SELECT numero, numero * numero AS quadrado FROM NumerosCTE
-- ✅ CTEs recursivas suportadas
```

### Caso de Uso 3: Tipos Diversos
```sql
SELECT 
    'Texto' AS string_col,
    42 AS int_col,
    3.14159 AS decimal_col,
    GETDATE() AS datetime_col,
    NEWID() AS uuid_col
-- ✅ Todos os tipos SQL Server suportados
```

## 🔍 **Troubleshooting**
### Erro Comum 1
**Sintoma**: Connection error
**Causa**: SQL Server não acessível
**Solução**: Verificar credenciais e conectividade

### Erro Comum 2
**Sintoma**: Query execution error
**Causa**: Sintaxe SQL inválida
**Solução**: Validar sintaxe SQL Server

### Erro Comum 3
**Sintoma**: JSON conversion error
**Causa**: Tipo não mapeado na conversão
**Solução**: Tipo é tratado por fallback inteligente

## 🔗 **Links Relacionados**
- Contextos relacionados: `CTX-Dynamic-Query-Support.md`
- ADRs relacionados: `ADR-002-Arquitetura-Rapido-SQL.md`
- Tasks relacionadas: `TASK-008-SQL-Server-Portal-Query.md`

## 🎯 **Arquitetura Técnica**
### Stack Tecnológica
```yaml
Linguagem: Rust
Driver SQL Server: Tiberius
Runtime: Tokio (async)
Conversão: serde_json
Pool: Não implementado (conexões diretas)
```

### Estratégia de Conversão
```rust
fn extract_value_static(row: &Row, column_name: &str) -> Result<Value> {
    // Tenta múltiplos tipos automaticamente:
    // 1. String (&str)
    // 2. Inteiros (i32, i64)
    // 3. Decimais (f64, Decimal)
    // 4. Booleanos (bool)
    // 5. Datas (NaiveDateTime)
    // 6. UUIDs (Uuid)
    // 7. Binários (&[u8])
    // 8. Fallback string genérica
}
```

### Tipos Suportados
| SQL Server | JSON | Tratamento |
|------------|------|------------|
| VARCHAR, NVARCHAR | string | Direto |
| INT, BIGINT | number | Direto |
| FLOAT, REAL | number | Direto |
| DECIMAL, MONEY | number/string | Preserva precisão |
| BIT | boolean | Direto |
| DATETIME | string | Formatado |
| UNIQUEIDENTIFIER | string | UUID |
| VARBINARY | string | Base64 |

## 📊 **Performance**
- **Conexão**: Direta (sem pool)
- **Processamento**: Stream assíncrono
- **Conversão**: Otimizada por tipo
- **Memória**: Baixo consumo
- **Latência**: < 100ms para queries simples

## 🚀 **Aplicação no FC Data API**
### Lições Extraídas
1. **Conversão cascata** é muito eficaz
2. **Fallback inteligente** evita erros
3. **Stream processing** para performance
4. **Tipos específicos** primeiro, genéricos depois

### Adaptações Necessárias
- Integrar com pool bb8-tiberius existente
- Adaptar validações de segurança do FC Data API
- Manter compatibilidade com endpoints existentes
- Adicionar logs estruturados

## 🔄 **Inspiração vs Integração**
### Opção 1: Integração Direta
```yaml
Prós: Solução pronta, funcionalidades completas
Contras: Dependência externa, precisa adaptar pool
Esforço: 2-3 horas
```

### Opção 2: Inspiração Arquitetural
```yaml
Prós: Controle total, integração perfeita
Contras: Mais implementação, possíveis bugs
Esforço: 4-6 horas
```

### Recomendação
**Inspiração Arquitetural**: Extrair conceitos de conversão cascata e adaptar para estrutura existente do FC Data API.

---
📅 **Criado**: 08/08/2025  
📅 **Atualizado**: 08/08/2025  
🔄 **Próxima revisão**: Após implementação no Portal
