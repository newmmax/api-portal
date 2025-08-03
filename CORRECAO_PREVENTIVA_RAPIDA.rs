// 🚀 CORREÇÃO PREVENTIVA RÁPIDA - Aplicar no src/handlers/portal_handlers.rs
// Substitua a função convert_sqlserver_value_to_json() (linhas ~260-390) por esta versão:

/// 🔧 CORREÇÃO PREVENTIVA: Conversão ultra-robusta com tratamento NULL
fn convert_sqlserver_value_to_json(row: &Row, col_index: usize, sql_type: tiberius::ColumnType) -> serde_json::Value {
    use tiberius::time::chrono::NaiveDateTime;
    use tiberius::ColumnType;
    
    // 🛡️ TRATAMENTO PREVENTIVO: Verificar NULL primeiro para TODOS os tipos
    match sql_type {
        // 🔢 TIPOS NUMÉRICOS com NULL-safety
        ColumnType::Int1 => {
            if let Ok(Some(v)) = row.try_get::<Option<bool>, _>(col_index) {
                json!(v)
            } else if let Ok(Some(v)) = row.try_get::<Option<u8>, _>(col_index) {
                json!(v)
            } else {
                json!(null)
            }
        },
        
        ColumnType::Int2 => {
            row.try_get::<Option<i16>, _>(col_index)
                .map(|opt| opt.map(json!).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
            
        ColumnType::Int4 => {
            row.try_get::<Option<i32>, _>(col_index)
                .map(|opt| opt.map(json!).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
            
        ColumnType::Int8 => {
            row.try_get::<Option<i64>, _>(col_index)
                .map(|opt| opt.map(json!).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
            
        ColumnType::Float4 => {
            row.try_get::<Option<f32>, _>(col_index)
                .map(|opt| opt.map(json!).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
            
        ColumnType::Float8 => {
            row.try_get::<Option<f64>, _>(col_index)
                .map(|opt| opt.map(json!).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
            
        // 💰 TIPOS MONETÁRIOS E DECIMAIS com NULL-safety
        ColumnType::Money => {
            if let Ok(Some(v)) = row.try_get::<Option<rust_decimal::Decimal>, _>(col_index) {
                json!(v.to_string())
            } else if let Ok(Some(v)) = row.try_get::<Option<f64>, _>(col_index) {
                json!(v)
            } else {
                json!(null)
            }
        },
        
        ColumnType::Decimaln | ColumnType::Numericn => {
            if let Ok(Some(v)) = row.try_get::<Option<rust_decimal::Decimal>, _>(col_index) {
                json!(v.to_string())
            } else if let Ok(Some(v)) = row.try_get::<Option<f64>, _>(col_index) {
                json!(v)
            } else {
                json!(null)
            }
        },
            
        // 📝 TIPOS TEXTO com NULL-safety
        ColumnType::NChar | ColumnType::NVarchar | ColumnType::NText => {
            row.try_get::<Option<&str>, _>(col_index)
                .map(|opt| opt.map(|v| json!(v.trim())).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
        
        ColumnType::Text => {
            row.try_get::<Option<&str>, _>(col_index)
                .map(|opt| opt.map(|v| json!(v.trim())).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
        
        // 📅 TIPOS DATA/HORA com NULL-safety ULTRA-ROBUSTA
        ColumnType::Datetime | ColumnType::Datetime2 => {
            row.try_get::<Option<NaiveDateTime>, _>(col_index)
                .map(|opt| opt.map(|dt| json!(dt.format("%Y-%m-%d %H:%M:%S").to_string())).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
        
        ColumnType::Daten => {
            row.try_get::<Option<chrono::NaiveDate>, _>(col_index)
                .map(|opt| opt.map(|d| json!(d.to_string())).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
        
        ColumnType::Timen => {
            row.try_get::<Option<chrono::NaiveTime>, _>(col_index)
                .map(|opt| opt.map(|t| json!(t.to_string())).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
                
        // 🔐 TIPOS IDENTIFICADORES com NULL-safety
        ColumnType::Guid => {
            row.try_get::<Option<uuid::Uuid>, _>(col_index)
                .map(|opt| opt.map(|id| json!(id.to_string())).unwrap_or(json!(null)))
                .unwrap_or(json!(null))
        },
                
        // 📦 TIPOS BINÁRIOS com NULL-safety
        ColumnType::Image => {
            if let Ok(Some(v)) = row.try_get::<Option<&[u8]>, _>(col_index) {
                use base64::Engine;
                json!(base64::engine::general_purpose::STANDARD.encode(v))
            } else {
                json!(null)
            }
        },
        
        // 📊 FALLBACK ULTRA-ROBUSTO com NULL-safety
        _ => {
            // 🛡️ ESTRATÉGIA CASCATA com try_get para evitar panics
            
            // 1️⃣ Primeira tentativa: String com NULL-safety
            if let Ok(opt_str) = row.try_get::<Option<String>, _>(col_index) {
                return opt_str.map(|s| json!(s.trim())).unwrap_or(json!(null));
            }
            
            // 2️⃣ Segunda tentativa: &str com NULL-safety
            if let Ok(opt_str) = row.try_get::<Option<&str>, _>(col_index) {
                return opt_str.map(|s| json!(s.trim())).unwrap_or(json!(null));
            }
            
            // 3️⃣ Terceira tentativa: Números inteiros com NULL-safety
            if let Ok(opt_i32) = row.try_get::<Option<i32>, _>(col_index) {
                return opt_i32.map(json!).unwrap_or(json!(null));
            }
            if let Ok(opt_i64) = row.try_get::<Option<i64>, _>(col_index) {
                return opt_i64.map(json!).unwrap_or(json!(null));
            }
            
            // 4️⃣ Quarta tentativa: Números decimais com NULL-safety
            if let Ok(opt_f64) = row.try_get::<Option<f64>, _>(col_index) {
                return opt_f64.map(json!).unwrap_or(json!(null));
            }
            
            // 5️⃣ Quinta tentativa: Booleano com NULL-safety
            if let Ok(opt_bool) = row.try_get::<Option<bool>, _>(col_index) {
                return opt_bool.map(json!).unwrap_or(json!(null));
            }
            
            // 6️⃣ Sexta tentativa: Data/hora com NULL-safety
            if let Ok(opt_dt) = row.try_get::<Option<NaiveDateTime>, _>(col_index) {
                return opt_dt.map(|dt| json!(dt.format("%Y-%m-%d %H:%M:%S").to_string())).unwrap_or(json!(null));
            }
            
            // 7️⃣ Sétima tentativa: UUID com NULL-safety
            if let Ok(opt_uuid) = row.try_get::<Option<uuid::Uuid>, _>(col_index) {
                return opt_uuid.map(|id| json!(id.to_string())).unwrap_or(json!(null));
            }
            
            // 8️⃣ Oitava tentativa: Decimal com NULL-safety
            if let Ok(opt_decimal) = row.try_get::<Option<rust_decimal::Decimal>, _>(col_index) {
                return opt_decimal.map(|d| json!(d.to_string())).unwrap_or(json!(null));
            }
            
            // 9️⃣ Nona tentativa: Binário com NULL-safety
            if let Ok(opt_bytes) = row.try_get::<Option<&[u8]>, _>(col_index) {
                if let Some(bytes) = opt_bytes {
                    use base64::Engine;
                    return json!(base64::engine::general_purpose::STANDARD.encode(bytes));
                } else {
                    return json!(null);
                }
            }
            
            // 🆘 ÚLTIMO RECURSO: Tipo completamente desconhecido
            log::warn!("Tipo SQL Server não suportado: {:?} na coluna {} - retornando null", sql_type, col_index);
            json!(null)
        }
    }
}
