# 🎯 TASK-CRÍTICA: Implementar Correção Cirúrgica Portal Query

## 📋 **Informações da Tarefa**
- **ID**: task-correcao-portal-select-asterisco
- **Prioridade**: 🔴 **CRÍTICA**
- **Prazo**: IMEDIATO - Próximo chat
- **Estimativa**: 2 horas
- **Status**: ⏳ **PENDENTE**
- **Categoria**: Backend - Correção Específica
- **Dependência**: Nenhuma - Scripts prontos

## ⚠️ **LOCALIZAÇÃO CORRETA DO PROJETO**
```yaml
PASTA CORRETA: D:\PROJETOS\RUST\fc-data-api\
❌ NÃO MEXER EM: C:\XAMPP\htdocs\portaldepedidos\fc-data-api
❌ NÃO MEXER EM: Outras pastas antigas

SEMPRE CONFIRMAR: 
✅ MCP load_project_plan("D:\PROJETOS\RUST\fc-data-api")
✅ Verificar existência de src/handlers/portal_handlers.rs
```

## 🎯 **Objetivo**
Resolver erro 502 específico do endpoint `/portal/query` quando usado `SELECT * FROM clientes`. Problema confirmado: conversão de tipos específicos na função `convert_sqlserver_value_to_json()`.

**RESULTADO ESPERADO**: 
- ✅ `SELECT nome FROM clientes` - Continua funcionando (83 registros)
- ✅ `SELECT * FROM clientes` - Passa a funcionar
- ✅ Query original complexa do usuário - Funciona perfeitamente

## 📋 **Checklist de Implementação**

### **FASE 1: Preparação e Identificação** ⏳
- [ ] 🔧 **Confirmar localização**: Verificar pasta D:\PROJETOS\RUST\fc-data-api\
- [ ] 🔧 **Carregar MCP**: load_project_plan("D:\PROJETOS\RUST\fc-data-api")
- [ ] 🧪 **Executar identificação**: .\IDENTIFICACAO_CIRURGICA_CAMPO.ps1
- [ ] 📝 **Documentar campo problemático**: Identificar qual campo específico causa erro

### **FASE 2: Implementação da Correção** ⏳
- [ ] 🎯 **Analisar código atual**: Examinar src/handlers/portal_handlers.rs
- [ ] 🔧 **Aplicar correção específica**: Baseada no campo identificado na Fase 1
- [ ] 🛡️ **Preservar funcionalidade**: Não quebrar SELECT nome que já funciona
- [ ] 🧪 **Validar localmente**: cargo build --release + teste local

### **FASE 3: Deploy e Validação** ⏳
- [ ] 📦 **Compilar release**: cargo build --release
- [ ] 💾 **Backup produção**: Salvar fc-data-api.exe atual
- [ ] 🚀 **Deploy seguro**: Copiar novo executável
- [ ] 🧪 **Teste produção**: Validar SELECT * funciona

## 🔧 **Especificações Técnicas**

### **Arquivo Principal**
```
D:\PROJETOS\RUST\fc-data-api\src\handlers\portal_handlers.rs
Função: convert_sqlserver_value_to_json() (linha ~217)
```

### **Scripts Diagnósticos Prontos**
```powershell
# Identificação completa:
D:\PROJETOS\RUST\fc-data-api\IDENTIFICACAO_CIRURGICA_CAMPO.ps1

# Identificação rápida:
D:\PROJETOS\RUST\fc-data-api\IDENTIFICACAO_RAPIDA_CAMPO.bat

# Plano de correção:
D:\PROJETOS\RUST\fc-data-api\PLANO_CORRECAO_ESPECIFICA.md
```

### **Campos Suspeitos (Ordem Probabilidade)**
```yaml
🚨 ALTA: 
  - deleted_at (DATETIME NULL)
  - is_first_login (BIT/BOOLEAN)  
  - created_at/updated_at (DATETIME)

🟡 MÉDIA:
  - id (BIGINT/INT PRIMARY KEY)
  - cnpj (VARCHAR com formatação)

🟢 BAIXA:
  - cod_totvs, loja, cidade, estado (VARCHAR normais)
```

### **Correções Específicas Preparadas**

#### **Para DATETIME NULL:**
```rust
ColumnType::Datetime | ColumnType::Datetime2 => {
    row.get::<Option<NaiveDateTime>, _>(col_index)
        .map(|opt| opt.map(|dt| json!(dt.format("%Y-%m-%d %H:%M:%S").to_string())).unwrap_or(json!(null)))
        .unwrap_or(json!(null))
},
```

#### **Para BIT/BOOLEAN:**
```rust
ColumnType::Bit => {
    row.get::<Option<bool>, _>(col_index)
        .map(|opt| opt.map(json!).unwrap_or(json!(null)))
        .unwrap_or(json!(null))
},
```

## 🎯 **Critérios de Sucesso**
- [ ] **Scripts identificação executados**: Campo problemático específico identificado
- [ ] **Correção aplicada**: Apenas no tipo/campo problemático
- [ ] **SELECT nome continua**: Funcionalidade existente preservada
- [ ] **SELECT * funciona**: Erro 502 resolvido
- [ ] **Query original funciona**: Teste completo com campos CONVERT
- [ ] **Deploy validado**: Produção funcionando sem regressões

## 📁 **Referências Criadas**
- **Contextos**: `docs/context/CTX-SQL-Server-Integration.md`
- **Investigação**: `IDENTIFICACAO_CIRURGICA_CAMPO.ps1`
- **Plano correção**: `PLANO_CORRECAO_ESPECIFICA.md`
- **Deploy**: `PREPARACAO_DEPLOY_COMPLETA.md`

## 🚨 **Comandos de Emergência**

### **Se der problema no deploy:**
```bash
# Rollback imediato:
nssm stop FCDataAPI
copy C:\fcdata-api\backup\fc-data-api-backup-*.exe C:\fcdata-api\fc-data-api.exe
nssm start FCDataAPI
```

### **Para validar funcionamento:**
```bash
# 1. Teste baseline:
curl -X POST https://conexao.artesanalfarmacia.com.br/services/api1/portal/query \
  -H "Authorization: Bearer [TOKEN]" \
  -d '{"query": "SELECT nome FROM clientes"}'

# 2. Teste corrção:
curl -X POST https://conexao.artesanalfarmacia.com.br/services/api1/portal/query \
  -H "Authorization: Bearer [TOKEN]" \
  -d '{"query": "SELECT * FROM clientes"}'
```

## 📋 **Log de Execução**
| Data/Hora | Ação | Status | Observações |
|-----------|------|--------|-------------|
| 01/08/2025 17:30 | Task criada | 📋 | Scripts diagnósticos prontos, problema confirmado |

## ⚠️ **AVISOS CRÍTICOS**
1. **SEMPRE verificar pasta**: D:\PROJETOS\RUST\fc-data-api\
2. **NUNCA mexer em**: C:\XAMPP\htdocs\portaldepedidos\fc-data-api
3. **EXECUTAR identificação primeiro**: Antes de aplicar qualquer correção
4. **Backup obrigatório**: Antes de deploy em produção
5. **Testar localmente**: Antes de subir para produção

---
📅 **Criado**: 01/08/2025  
⏭️ **Próxima sessão**: Executar imediatamente  
🎯 **Resultado**: SELECT * FROM clientes funcionando em produção
