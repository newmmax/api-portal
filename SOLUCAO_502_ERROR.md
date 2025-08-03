# Diagnóstico do Erro 502 - FC Data API

## Problema
- Erro 502 Proxy Error ao chamar `/portal/query`
- Apache não consegue resposta da API upstream
- Funciona no PostgreSQL mas falha no SQL Server

## Possíveis Causas

### 1. Timeout da Query SQL
- SQL Server pode estar demorando muito
- Apache tem timeout padrão de 30-60 segundos
- Query `SELECT * FROM clientes` pode ter muitos registros

### 2. Problema de Conexão com SQL Server
- Firewall entre servidor web e SQL Server (10.216.1.11)
- Pool de conexões esgotado
- Credenciais incorretas em produção

### 3. Erro no Handler
- Possível loop infinito ao processar resultados
- Memória insuficiente para resultado grande
- Erro de serialização JSON

## Soluções Recomendadas

### Solução Imediata 1: Query com LIMIT
```sql
-- Em vez de:
SELECT * FROM clientes

-- Use:
SELECT TOP 10 * FROM clientes
```

### Solução Imediata 2: Aumentar Timeout no Apache
Adicionar no VirtualHost:
```apache
ProxyTimeout 300
ProxyBadHeader Ignore
```

### Solução Imediata 3: Verificar se API está rodando
```batch
# No servidor
curl http://localhost:8089/services/api1/health

# Ver logs
type C:\fcdata-api\logs\service.log
```

## Script de Teste Completo