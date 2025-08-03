# 🧪 BASES DE TESTE - FC DATA API

## Configuração das Bases de Teste

Para desenvolvimento e testes, use as seguintes bases:

### SQL Server - Portal de Pedidos (TESTE)
```env
# Base de teste - pode criar/modificar pedidos livremente
PORTAL_DATABASE_NAME=sys_pedidos_teste
PORTAL_DATABASE_HOST=10.216.1.11
PORTAL_DATABASE_PORT=1433
PORTAL_DATABASE_USER=sa
PORTAL_DATABASE_PASS=5y54dm1n%
```

### SQL Server - Protheus ERP (TESTE)
```env
# Base de teste - dados de integração
PROTHEUS_DATABASE_NAME=sigaofc_teste
PROTHEUS_DATABASE_HOST=10.216.1.11
PROTHEUS_DATABASE_PORT=1433
PROTHEUS_DATABASE_USER=sa
PROTHEUS_DATABASE_PASS=5y54dm1n%
```

### PostgreSQL - FC Data (PRODUÇÃO)
```env
# Mantém produção pois é apenas leitura
DATABASE_URL=postgres://rodrigo:R0drigoPgSQL@10.216.1.16:5432/fc_data
```

## Como Alternar Entre Produção e Teste

1. **Para usar bases de TESTE**: Modifique o `.env`:
   ```bash
   PORTAL_DATABASE_NAME=syspedidos_teste
   PROTHEUS_DATABASE_NAME=sigaofc_teste
   ```

2. **Para voltar para PRODUÇÃO**:
   ```bash
   PORTAL_DATABASE_NAME=sys_pedidos
   PROTHEUS_DATABASE_NAME=sigaofc
   ```

3. **Reinicie a API** após mudar o `.env`

## ⚠️ IMPORTANTE

- **NUNCA** crie pedidos de teste em produção
- As bases de teste são resetadas periodicamente
- Use sempre CPF/CNPJ fictícios em testes
- Documente todos os testes realizados

## Exemplos de Dados de Teste

### Cliente Teste
```sql
-- Verificar clientes disponíveis na base teste
SELECT TOP 10 id, razao_social, cnpj, grupo_venda 
FROM sys_pedidos_teste.dbo.clientes
WHERE ativo = 1
```

### Produtos Teste
```sql
-- Produtos com saldo para teste
SELECT TOP 10 p.id, p.codigo, p.descricao, p.saldo, p.quantidade_minima_embalagem
FROM sys_pedidos_teste.dbo.produtos p
WHERE p.status = 1 AND p.saldo > 0
```
