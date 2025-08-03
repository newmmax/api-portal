# 🎯 PLANO DE CORREÇÃO ESPECÍFICA - Portal Query

## 📋 **Situação Confirmada**
```yaml
✅ SELECT nome FROM clientes: FUNCIONA (83 registros)
❌ SELECT * FROM clientes: ERRO 502
🎯 Causa: Conversão de tipos específicos na função convert_sqlserver_value_to_json()
```

## 🔍 **Campos Suspeitos (Ordem de Probabilidade)**

### **🚨 ALTA PROBABILIDADE**
1. **deleted_at** (DATETIME NULL) - Valores NULL podem não estar sendo tratados
2. **is_first_login** (BIT/BOOLEAN) - Tipo BIT pode não estar mapeado corretamente
3. **created_at/updated_at** (DATETIME) - Conversão datetime pode ter problema

### **🟡 MÉDIA PROBABILIDADE**  
4. **id** (BIGINT/INT com AUTO_INCREMENT) - Pode ser tipo específico SQL Server
5. **cnpj** (VARCHAR com formatação especial) - Valores com caracteres especiais

### **🟢 BAIXA PROBABILIDADE**
6. **cod_totvs, loja, cidade, estado, grupo_venda, nome_fantasia** (VARCHAR) - Similares ao nome que funciona

## 🛠️ **Correções Específicas Necessárias**

### **Correção 1: Valores NULL em DATETIME**
```rust
// No convert_sqlserver_value_to_json(), melhorar tratamento NULL:
ColumnType::Datetime | ColumnType::Datetime2 => {
    // Verificar explicitamente se é NULL primeiro
    if row.get::<Option<NaiveDateTime>, _>(col_index).is_none() {
        json!(null)
    } else if let Some(dt) = row.get::<NaiveDateTime, _>(col_index) {
        json!(dt.format("%Y-%m-%d %H:%M:%S").to_string())
    } else {
        json!(null)
    }
},
```

### **Correção 2: Tipo BIT/BOOLEAN**
```rust
// Adicionar tratamento específico para BIT:
ColumnType::Bit => {
    if let Some(v) = row.get::<bool, _>(col_index) {
        json!(v)
    } else if let Some(v) = row.get::<u8, _>(col_index) {
        json!(v != 0)  // Converter 0/1 para false/true
    } else {
        json!(null)
    }
},
```

### **Correção 3: Fallback Mais Robusto para NULL**
```rust
// No fallback cascata, adicionar verificação NULL em cada tentativa:
_ => {
    // 1️⃣ Verificar se é NULL explicitamente primeiro
    if row.try_get::<Option<String>, _>(col_index).unwrap_or(None).is_none() {
        return json!(null);
    }
    
    // 2️⃣ Tentar conversões existentes...
    if let Some(v) = row.get::<&str, _>(col_index) {
        return json!(v.trim());
    }
    // ... resto do fallback
}
```

## 🚀 **Plano de Implementação**

### **PASSO 1: Identificar Campo Exato**
```bash
# Execute para identificar qual campo específico:
.\IDENTIFICACAO_CIRURGICA_CAMPO.ps1
# OU mais rápido:
.\IDENTIFICACAO_RAPIDA_CAMPO.bat
```

### **PASSO 2: Aplicar Correção Específica**
Baseado no resultado, aplicar uma das correções acima no arquivo:
`D:\PROJETOS\RUST\fc-data-api\src\handlers\portal_handlers.rs`

### **PASSO 3: Compilar e Testar**
```bash
cd D:\PROJETOS\RUST\fc-data-api
cargo build --release
# Testar localmente primeiro
curl -X POST http://localhost:8089/services/api1/portal/query \
  -H "Authorization: Bearer [TOKEN]" \
  -d '{"query": "SELECT * FROM clientes"}'
```

### **PASSO 4: Deploy Seguro**
```bash
# Parar serviço
nssm stop FCDataAPI

# Backup
copy C:\fcdata-api\fc-data-api.exe C:\fcdata-api\backup\

# Deploy
copy target\release\fc-data-api.exe C:\fcdata-api\

# Iniciar
nssm start FCDataAPI
```

## 🔧 **Correção Preventiva (Se Identificação Não For Conclusiva)**

Se a identificação não for conclusiva, aplicar correção preventiva para **TODOS** os tipos problemáticos:

```rust
fn convert_sqlserver_value_to_json(row: &Row, col_index: usize, sql_type: tiberius::ColumnType) -> serde_json::Value {
    use tiberius::time::chrono::NaiveDateTime;
    use tiberius::ColumnType;
    
    // 🛡️ CORREÇÃO PREVENTIVA: Verificar NULL primeiro para TODOS os tipos
    match sql_type {
        ColumnType::Bit => {
            // Tratamento específico BIT
            row.get::<Option<bool>, _>(col_index)
                .map(|opt| opt.map(json!).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
        
        ColumnType::Datetime | ColumnType::Datetime2 => {
            // Tratamento robusto DATETIME com NULL
            row.get::<Option<NaiveDateTime>, _>(col_index)
                .map(|opt| opt.map(|dt| json!(dt.format("%Y-%m-%d %H:%M:%S").to_string())).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
        
        // ... outros tipos existentes ...
        
        _ => {
            // Fallback ainda mais robusto
            // 🛡️ PRIMEIRO: Tentar como Option<String> para detectar NULL
            if let Ok(opt_str) = row.try_get::<Option<String>, _>(col_index) {
                return opt_str.map(|s| json!(s.trim())).unwrap_or(json!(null));
            }
            
            // Se falhar, continuar com fallback existente...
            // ... resto do código existente ...
        }
    }
}
```

## 📊 **Validação Pós-Correção**

### **Testes Obrigatórios**
```bash
# 1. Campo que funcionava antes continua funcionando:
SELECT nome FROM clientes

# 2. Campo problemático agora funciona:
SELECT nome, [CAMPO_IDENTIFICADO] FROM clientes

# 3. SELECT * completo agora funciona:
SELECT * FROM clientes

# 4. Query original do usuário:
SELECT id, cod_totvs, loja, nome, email, cnpj, cidade, estado, 
       CONVERT(varchar, created_at, 120) as created_at, 
       CONVERT(varchar, updated_at, 120) as updated_at, 
       CONVERT(varchar, deleted_at, 120) as deleted_at, 
       CAST(is_first_login as int) as is_first_login, 
       grupo_venda, nome_fantasia 
FROM clientes
```

## 🎯 **Resultado Esperado**

Após aplicar a correção específica:
- ✅ `SELECT nome FROM clientes` - Continua funcionando  
- ✅ `SELECT * FROM clientes` - Passa a funcionar
- ✅ Query original complexa - Funciona perfeitamente
- ✅ Sem quebrar outros endpoints

---

**📅 Preparado**: 01/08/2025  
**🎯 Status**: Scripts prontos para identificação + correções específicas preparadas  
**🚀 Próximo**: Execute IDENTIFICACAO_CIRURGICA_CAMPO.ps1 → Aplique correção → Compile → Deploy
