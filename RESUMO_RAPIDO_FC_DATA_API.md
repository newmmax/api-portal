# FC Data API - Referência Rápida

## 🎯 O que é?
API REST em Rust para consultar dados de vendas do sistema FC (PostgreSQL).

## 📍 Onde está?
- **Local**: `C:\XAMPP\htdocs\portaldepedidos\fc-data-api`
- **Produção**: `https://conexao.artesanalfarmacia.com.br/services/api1`
- **Dev**: `http://localhost:8089/services/api1`

## 🔑 Acesso Rápido
```yaml
# PostgreSQL
Host: 10.216.1.16:5432
DB: fc_data
User: rodrigo
Pass: R0drigoPgSQL

# API Admin
User: admin
Pass: ArtesanalFC2025!

# Porta Local: 8089
```

## 📡 Endpoints Principais
```bash
# Health Check (público)
GET /services/api1/health

# Login (público)
POST /services/api1/auth/login
Body: {"username":"admin","password":"ArtesanalFC2025!"}

# Consultar Vendas (precisa token)
GET /services/api1/data/vendas?data_inicio=2025-01-01&limite=10
Header: Authorization: Bearer {token}
```

## 🚀 Deploy Rápido
1. Copiar pasta `temp_deploy` para servidor
2. Executar como Admin:
   - `01_VALIDACAO_MENU.bat`
   - `02_BACKUP_ATUAL.bat`
   - `03_DEPLOY_PASSO_A_PASSO.bat`
   - `04_VALIDACAO_FINAL.bat`

## 🔧 Comandos Úteis
```bash
# Compilar
cargo build --release

# Status do serviço
sc query FCDataAPI

# Ver logs
type C:\fcdata-api\logs\service.log

# Testar
curl http://localhost:8089/services/api1/health
```

## ⚠️ Problemas Comuns
- **Porta em uso**: Mudar SERVER_PORT no .env
- **Token expirado**: Fazer novo login
- **Serviço não inicia**: Ver logs em C:\fcdata-api\logs\

## 📄 Arquivos Importantes
- `CONTEXTO_COMPLETO_FC_DATA_API.md` - Documentação completa
- `temp_deploy/` - Pasta pronta para deploy
- `.env` - Configurações (editar antes de produção!)
- `deploy-seguro/` - Scripts de instalação

---
**Stack**: Rust + Actix-Web + PostgreSQL + JWT + Windows Service
