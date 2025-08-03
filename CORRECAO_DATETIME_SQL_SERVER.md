# Correção DateTime SQL Server - FC Data API

## Data: 15/01/2025

## Problema Resolvido
- Erro 502 ao fazer queries que retornam campos datetime do SQL Server
- Causa: Serialização incorreta de datetime para JSON

## Solução Implementada

### 1. Correção no Código (portal_handlers.rs)
Adicionado tratamento específico para campos datetime:
```rust
tiberius::ColumnType::Datetime | tiberius::ColumnType::Datetime2 => {
    row.get::<NaiveDateTime, _>(i)
        .map(|dt| json!(dt.format("%Y-%m-%d %H:%M:%S").to_string()))
        .unwrap_or(json!(null))
}
```

### 2. Para Queries Customizadas

Agora funciona de duas formas:

**Opção A - Query normal (após a correção):**
```json
{
  "sql": "SELECT * FROM clientes"
}
```

**Opção B - Com CONVERT (ainda funciona):**
```json
{
  "sql": "SELECT id, nome, CONVERT(varchar, created_at, 120) as created_at FROM clientes"
}
```

## Compilar e Testar
```batch
cd C:\XAMPP\htdocs\portaldepedidos\fc-data-api
cargo build --release
```

## Benefícios
- Não precisa mais usar CONVERT nas queries
- Datetime é automaticamente formatado como "YYYY-MM-DD HH:MM:SS"
- Mantém compatibilidade com queries existentes