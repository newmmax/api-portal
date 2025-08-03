# Contexto para IA: FC Data API

Estou trabalhando com uma API REST em Rust chamada FC Data API, localizada em `C:\XAMPP\htdocs\portaldepedidos\fc-data-api`.

## Resumo do Projeto:
- **O que é**: API para consultar dados de vendas de um sistema ERP
- **Stack**: Rust + Actix-Web 4 + PostgreSQL + JWT
- **Banco**: PostgreSQL remoto em 10.216.1.16:5432
- **Deploy**: Windows Service com NSSM + Apache proxy reverso
- **URL Produção**: https://conexao.artesanalfarmacia.com.br/services/api1

## Query Principal:
A API executa uma query complexa que junta 7 tabelas (FC14000, FC14100, fc03000, fc07000, fc08000, companies, company_config) para retornar dados detalhados de vendas com informações de empresa, filial, cliente, vendedor e produtos.

## Estrutura de Código:
```
src/
├── main.rs (entrada)
├── auth.rs (JWT middleware)
├── config.rs (configurações)
├── models.rs (structs)
└── handlers/ (endpoints)
```

## Endpoints Principais:
- POST /auth/login - Login (user: admin, pass: ArtesanalFC2025!)
- GET /data/vendas - Consulta vendas (precisa JWT token)
- GET /health - Status da API

## Estado Atual:
- ✅ API desenvolvida e testada localmente
- ✅ Sistema de deploy seguro criado
- ✅ Executável compilado (6.3 MB)
- ⏳ Aguardando deploy em produção

## Problemas Comuns:
1. Porta 8089 em uso - mudar no .env
2. Token JWT expira em 24h - fazer novo login
3. Campos monetários no banco são REAL - precisam CAST para numeric

## Deploy:
Pasta `temp_deploy` contém tudo necessário:
- fc-data-api.exe
- .env (configurações)
- deploy-seguro/ (scripts de instalação)

Scripts executam validações, backup e instalação passo a passo com rollback automático.

## Arquivos de Referência:
- `CONTEXTO_COMPLETO_FC_DATA_API.md` - Documentação detalhada
- `CONTEXTO_FC_DATA_API.json` - Dados estruturados
- `temp_deploy/` - Pronto para produção

Se precisar de mais detalhes sobre qualquer aspecto, os arquivos de contexto contêm informações completas.
