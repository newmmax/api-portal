# Configuração de Desenvolvimento/Teste para FC Data API

## Ambientes Disponíveis

### PRODUÇÃO (atual no .env)
```
PORTAL_DATABASE_NAME=sys_pedidos
PROTHEUS_DATABASE_NAME=sigaofc
```

### TESTE (usar quando necessário)
```
PORTAL_DATABASE_NAME=syspedidos_teste
PROTHEUS_DATABASE_NAME=sigaofc_teste
```

## Como Alternar Entre Ambientes

1. **Para usar bases de TESTE:**
   - Edite o arquivo `.env`
   - Mude `sys_pedidos` para `syspedidos_teste`
   - Mude `sigaofc` para `sigaofc_teste`
   - Atualize também as connection strings

2. **Para voltar para PRODUÇÃO:**
   - Reverta as mudanças acima

## Exemplo de .env para TESTE
```env
# SQL Server - Portal de Pedidos (TESTE)
PORTAL_DATABASE_NAME=syspedidos_teste
PORTAL_CONNECTION_STRING=Server=tcp:10.216.1.11,1433;Database=syspedidos_teste;UID=sa;PWD=5y54dm1n%;TrustServerCertificate=true

# SQL Server - Protheus ERP (TESTE)
PROTHEUS_DATABASE_NAME=sigaofc_teste
PROTHEUS_CONNECTION_STRING=Server=tcp:10.216.1.11,1433;Database=sigaofc_teste;UID=sa;PWD=5y54dm1n%;TrustServerCertificate=true
```

## Importante
- As senhas são as MESMAS em produção e teste
- Use bases de teste para desenvolvimento e testes
- NUNCA crie pedidos de teste em produção
