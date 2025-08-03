# 🚀 COMPILAÇÃO E DEPLOY - FC Data API

## 📋 **Checklist Pré-Compilação**

### ✅ **Status Investigação Profissional**
```yaml
Descobertas:
  ✅ Endpoint /portal/query: FUNCIONANDO (query simples)
  ✅ Handler Rust: Código robusto e bem implementado
  ✅ Conversão tipos SQL Server: Função completa
  ✅ Conexão SQL Server: Operacional

Problema Refinado:
  ❌ Query específica complexa do usuário: SELECT id, cod_totvs, loja, nome...
  🎯 Causa Provável: Campo específico ou timeout na query complexa
  
Scripts Diagnóstico Criados:
  ✅ INVESTIGACAO_PORTAL_PROFISSIONAL.ps1 - Teste incremental
  ✅ INVESTIGACAO_PORTAL_PROFISSIONAL.bat - Alternativa CMD
```

### 🔍 **Investigação Realizada**
- ✅ Código analisado: `src/handlers/portal_handlers.rs`
- ✅ Função `convert_sqlserver_value_to_json()`: Muito robusta
- ✅ Fallback cascata: 8 níveis de conversão
- ✅ Tratamento tipos SQL Server: Completo

### 🎯 **Suspeitas Principais**
1. **Query Timeout**: Query complexa demora muito para executar
2. **Campo Específico**: Algum campo com valor NULL/problemático
3. **Limite Dados**: Query retorna muitos registros
4. **Tipo Específico**: Campo com tipo não tratado corretamente

## 🛠️ **Próximos Passos de Compilação**

### **1. Execute Investigação (PRIMEIRO)**
```powershell
# No servidor ou local:
cd D:\PROJETOS\RUST\fc-data-api
.\INVESTIGACAO_PORTAL_PROFISSIONAL.ps1
```

### **2. Compile Após Identificar Problema**
```batch
# Compilação release:
cd D:\PROJETOS\RUST\fc-data-api
cargo build --release

# Verificar build:
dir target\release\fc-data-api.exe
```

### **3. Deploy Seguro**
```batch
# Parar serviço:
nssm stop FCDataAPI

# Backup atual:
copy C:\fcdata-api\fc-data-api.exe C:\fcdata-api\backup\fc-data-api-backup-%date%.exe

# Deploy novo:
copy target\release\fc-data-api.exe C:\fcdata-api\fc-data-api.exe

# Iniciar serviço:
nssm start FCDataAPI

# Verificar status:
sc query FCDataAPI
```

## 🔧 **Possíveis Correções Baseadas na Investigação**

### **Se Problema = Timeout Query**
```rust
// Em src/handlers/portal_handlers.rs, adicionar timeout:
use tokio::time::{timeout, Duration};

let result = timeout(
    Duration::from_secs(300), // 5 minutos
    query_obj.query(&mut conn)
).await
.map_err(|_| ApiError::Database("Query timeout (300s)".to_string()))?
.map_err(|e| ApiError::Database(format!("Erro ao executar query: {}", e)))?;
```

### **Se Problema = Campo Específico**
```rust
// Adicionar mais logs na função convert_sqlserver_value_to_json:
fn convert_sqlserver_value_to_json(row: &Row, col_index: usize, sql_type: tiberius::ColumnType) -> serde_json::Value {
    log::debug!("Convertendo coluna {} tipo {:?}", col_index, sql_type);
    
    // ... código existente ...
    
    // No fallback, adicionar mais detalhes:
    _ => {
        log::warn!("Tipo SQL Server não suportado: {:?} na coluna {} - tentando fallback", sql_type, col_index);
        // ... fallback existente ...
    }
}
```

### **Se Problema = Limite de Dados**
```rust
// Adicionar limite de segurança nas queries:
pub static MAX_ROWS_LIMIT: usize = 10000;

// No handler, após coletar rows:
if rows.len() > MAX_ROWS_LIMIT {
    log::warn!("Query retornou {} registros, limitando a {}", rows.len(), MAX_ROWS_LIMIT);
    rows.truncate(MAX_ROWS_LIMIT);
}
```

## 📊 **Configurações de Deploy**

### **Arquivo .env Produção**
```bash
# Verificar configurações em .env.production:
RUST_LOG=info  # ou debug se precisar mais detalhes
SERVER_PORT=8089
SQLSERVER_PORTAL_URL=server=10.216.1.16,1433;database=portal;user=usuario;password=senha;TrustServerCertificate=true
```

### **Configuração Apache**
```apache
# Aumentar timeout se necessário:
ProxyTimeout 300
ProxyPass /services/api1 http://localhost:8089/services/api1 timeout=300
ProxyPassReverse /services/api1 http://localhost:8089/services/api1
```

## 🚨 **Plano de Rollback**

Se deploy der problema:
```batch
# Parar serviço:
nssm stop FCDataAPI

# Restaurar backup:
copy C:\fcdata-api\backup\fc-data-api-backup-*.exe C:\fcdata-api\fc-data-api.exe

# Reiniciar:
nssm start FCDataAPI
```

## 📝 **Validação Pós-Deploy**

### **Testes Obrigatórios**
```bash
# 1. Health check:
curl https://conexao.artesanalfarmacia.com.br/services/api1/health

# 2. Login:
curl -X POST https://conexao.artesanalfarmacia.com.br/services/api1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin_prod","password":"Pr0duc@0_FC_2025!Art3s@n@l"}'

# 3. Query simples Portal:
curl -X POST https://conexao.artesanalfarmacia.com.br/services/api1/portal/query \
  -H "Authorization: Bearer [TOKEN]" \
  -H "Content-Type: application/json" \
  -d '{"sql": "SELECT 1 as test"}'

# 4. Query original (após fix):
curl -X POST https://conexao.artesanalfarmacia.com.br/services/api1/portal/query \
  -H "Authorization: Bearer [TOKEN]" \
  -H "Content-Type: application/json" \
  -d '{"sql": "SELECT id, cod_totvs, loja, nome FROM clientes"}'
```

---

**📅 Preparado**: 01/08/2025  
**🎯 Status**: Pronto para investigação → correção → compilação → deploy  
**🚀 Próximo**: Execute INVESTIGACAO_PORTAL_PROFISSIONAL.ps1 para identificar problema específico
