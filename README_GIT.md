# 🦀 FC Data API - Portal de Pedidos

**API REST em Rust para sistema de pedidos e analytics de franquias**

## 🎯 Visão Geral

API robusta desenvolvida em Rust usando Actix-Web 4, com integração dupla:
- **PostgreSQL**: Dados históricos FC
- **SQL Server**: Sistema de pedidos atual

## 🚀 Features Principais

### 📊 **Analytics Inteligentes**
- **Card 01**: Recompra Inteligente - IA para sugestões baseadas em padrões
- **Card 02**: Oportunidades na Rede - Comparação com benchmark ABC
- **Portal Query**: Consultas dinâmicas SQL Server

### 🔐 **Segurança**
- Autenticação JWT robusta
- Logs detalhados para auditoria
- Validação de entrada em todos endpoints

### ⚡ **Performance**
- Pool de conexões otimizado
- Consultas com índices apropriados
- Executável compilado com otimizações máximas

## 🏗️ Arquitetura

```
src/
├── main.rs                 # Entrada principal
├── auth.rs                 # Middleware JWT
├── config.rs               # Configurações
├── database.rs             # Pool de conexões
├── models.rs               # Modelos de dados
└── handlers/               # Lógica de negócio
    ├── auth_handlers.rs    # Autenticação
    ├── data_handlers.rs    # Dados FC (PostgreSQL)
    ├── portal_handlers.rs  # Sistema pedidos (SQL Server)
    ├── analytics_handlers.rs # Cards inteligentes
    └── query_handlers.rs   # Consultas customizadas
```

## 🚀 Quick Start

### Development
```bash
cargo run
# API rodando em http://localhost:8089/services/api1
```

### Production Build
```bash
cargo build --release
# Executável em target/release/fc-data-api.exe
```

### Deploy Windows Service
```bash
.\deploy-seguro\01_VALIDACAO_MENU.bat
.\deploy-seguro\02_BACKUP_ATUAL.bat
.\deploy-seguro\03_DEPLOY_PASSO_A_PASSO.bat
.\deploy-seguro\04_VALIDACAO_FINAL.bat
```

## 📡 Endpoints Principais

### Públicos
- `GET /health` - Status da API
- `POST /auth/login` - Autenticação JWT

### Autenticados
- `GET /data/vendas` - Consultar vendas (PostgreSQL)
- `POST /portal/query` - Query dinâmica (SQL Server)
- `GET /analytics/recompra` - Card 01: Sugestões IA
- `GET /analytics/oportunidades` - Card 02: Benchmark rede

## 🔧 Configuração

### Environment (.env)
```env
# Servidor
SERVER_HOST=0.0.0.0
SERVER_PORT=8089

# JWT
JWT_SECRET=your_super_secret_key
JWT_EXPIRES_IN=86400

# PostgreSQL (FC Data)
FC_DATABASE_URL=postgres://user:pass@host:5432/fc_data

# SQL Server (Portal)
PORTAL_DATABASE_URL=...
```

## 🛠️ Tecnologias

- **Rust**: 1.75+
- **Actix-Web**: 4.x
- **PostgreSQL**: Dados históricos
- **SQL Server**: Sistema atual
- **JWT**: Autenticação
- **Serde**: Serialização JSON
- **Tokio**: Runtime async

## 📊 Status do Projeto

- ✅ **Core API**: Desenvolvida e testada
- ✅ **Autenticação**: JWT implementado
- ✅ **Banco duplo**: PostgreSQL + SQL Server
- ⏳ **Cards Analytics**: Em desenvolvimento
- ⏳ **Deploy Produção**: Preparado

## 🤝 Contribuição

Este é um projeto interno da Artesanal Farmácia.

## 📄 Licença

Proprietário - Artesanal Farmácia

---

**Desenvolvido com ❤️ e ⚡ em Rust**
