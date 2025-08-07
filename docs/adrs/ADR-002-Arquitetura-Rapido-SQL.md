# ADR-002: Adoção da Arquitetura Rapido-SQL para Queries Dinâmicas

## Status
**ACEITO** - 08/08/2025

## Contexto
O FC Data API possui limitações críticas nos endpoints de query personalizada:

### Problemas Identificados
- **SELECT *** falha completamente em todos os endpoints
- **WITH (CTEs)** não funcionam
- **CASE statements** complexos falham
- **JOINs dinâmicos** com tipos desconhecidos falham
- Sistema atual mapeia para structs Rust fixas, não consegue lidar com tipos dinâmicos

### Situação por Endpoint
```yaml
/data/query (PostgreSQL): ❌ Limitado
/portal/query (SQL Server Portal): ❌ Limitado  
/protheus/query (SQL Server Protheus): ❌ Limitado
```

### Impacto no Negócio
- Analistas não conseguem fazer queries exploratórias
- Relatórios complexos impossíveis de gerar
- Portal de Pedidos limitado a queries pré-definidas
- Perda de flexibilidade para análise de dados

## Decisão
**Adotar a arquitetura de conversão dinâmica da Rapido-SQL** como base para resolver os problemas de queries complexas.

### Implementação Escolhida
1. **PostgreSQL**: Inspiração arquitetural - criar dynamic_query_handler.rs
2. **SQL Server**: Integração/adaptação direta da Rapido-SQL crate
3. **Endpoints paralelos**: Manter endpoints originais + criar novos dinâmicos

## Alternativas Consideradas

### Alternativa 1: Refatorar Sistema Atual
**Prós:**
- Mantém arquitetura existente
- Sem dependências externas

**Contras:**
- Muito trabalho de re-implementação
- Alto risco de quebrar funcionalidades existentes
- Não resolve problemas fundamentais de design

### Alternativa 2: Migrar Completamente para Rapido-SQL
**Prós:**
- Solução completa e testada
- Funcionalidades avançadas

**Contras:**
- Quebra compatibilidade total
- Necessário reescrever todo sistema de queries
- Alto risco de regressões

### Alternativa 3: Arquitetura Híbrida (ESCOLHIDA)
**Prós:**
- Zero risco para funcionalidades existentes
- Rollback simples se necessário
- Permite validação gradual
- Melhor dos dois mundos

**Contras:**
- Duplicação temporária de código
- Manutenção de dois sistemas

## Consequências

### Positivas
- **Resolução completa** dos problemas de SELECT * e queries complexas
- **Zero risco** para funcionalidades existentes
- **Flexibilidade total** para analistas e relatórios
- **Arquitetura escalável** para futuros bancos de dados
- **Performance mantida** ou melhorada
- **Documentação rica** e troubleshooting

### Negativas
- **Overhead de manutenção** de endpoints paralelos (temporário)
- **Curva de aprendizado** para desenvolvedores
- **Testes adicionais** necessários
- **Documentação duplicada** (temporário)

### Neutras
- **Mudança gradual** na forma de fazer queries
- **Coexistência** de endpoints antigos e novos
- **Opção de migração** futura para endpoints únicos

## Implementação

### Fase 1: PostgreSQL (CONCLUÍDA ✅)
```yaml
Arquivo: src/handlers/dynamic_query_handler.rs
Endpoint: POST /data/query-dynamic
Funcionalidade: SELECT *, CTEs, CASE, JOINs
Status: Implementado e testado
Duração: 4 horas
```

### Fase 2: SQL Server Portal (PRÓXIMA)
```yaml
Abordagem: Integração/adaptação Rapido-SQL
Endpoint: POST /portal/query-dynamic
Funcionalidade: SELECT *, CTEs, CASE, JOINs para Portal
Estimativa: 2-3 horas
```

### Fase 3: SQL Server Protheus (FUTURA)
```yaml
Abordagem: Reutilizar solução Portal
Endpoint: POST /protheus/query-dynamic
Funcionalidade: SELECT *, CTEs, CASE, JOINs para Protheus
Estimativa: 1-2 horas
```

### Fase 4: Consolidação (OPCIONAL)
```yaml
Decisão Futura: Migrar endpoints originais ou manter paralelos
Critério: Após 6 meses de uso sem problemas
Benefício: Simplificação da arquitetura
```

## Compliance
### Como Garantir Implementação
- **Code review** obrigatório para mudanças em query handlers
- **Testes automatizados** para queries complexas
- **Documentação atualizada** a cada nova implementação
- **Monitoramento** de performance e erros

### Métricas de Sucesso
- **SELECT *** funcionando em 100% dos casos
- **Latência** < 300ms para queries complexas
- **Zero regressões** em funcionalidades existentes
- **Redução de 90%** em tickets de "query não funciona"

## Riscos e Mitigações
### Risco 1: Performance Degradada
**Mitigação**: Benchmarks contínuos, otimizações na conversão

### Risco 2: Bugs na Conversão de Tipos
**Mitigação**: Testes extensivos, fallback para tipos desconhecidos

### Risco 3: Complexidade de Manutenção
**Mitigação**: Documentação rica, código bem estruturado

### Risco 4: Resistência dos Desenvolvedores
**Mitigação**: Treinamento, documentação clara, benefícios evidentes

## Precedentes
Esta decisão estabelece o padrão para futuras integrações:
- **Endpoints paralelos** para mudanças arriscadas
- **Arquitetura híbrida** para compatibilidade
- **Inspiração em soluções externas** quando apropriado

## Referências
- Crate Rapido-SQL: `D:\PROJETOS\RAPIDO\rapido-sql`
- Implementação PostgreSQL: `src/handlers/dynamic_query_handler.rs`
- Documentação: `DYNAMIC_QUERY_GUIDE.md`
- Contextos: `CTX-Dynamic-Query-Support.md`, `CTX-Rapido-SQL.md`

---
**Decisão tomada**: 08/08/2025  
**Implementação**: Fase 1 concluída, Fase 2 em andamento  
**Revisão prevista**: 08/02/2026 (6 meses)
