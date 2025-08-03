# 🦀 FC Data API - Portal de Pedidos + Cards Analytics

**API REST em Rust para sistema de pedidos e analytics inteligentes de franquias**

## 🎯 Visão Geral

API robusta desenvolvida em Rust usando Actix-Web 4, com integração tripla:
- **PostgreSQL**: Dados históricos FC
- **SQL Server Portal**: Sistema de pedidos atual
- **SQL Server Protheus**: Dados ERP corporativo

## 🚀 Features Principais

### 🔥 **CARDS ANALYTICS (NOVO!)**
- **🔄 Card 01: Recompra Inteligente** - IA para sugestões baseadas em padrões históricos
- **🏆 Card 02: Oportunidades na Rede** - Comparação com benchmark ABC da rede
- **🎯 Algoritmos avançados** - Score inteligente e priorização automática
- **💡 Insights personalizados** - Mensagens UX prontas para interface

### 📊 **Analytics Inteligentes**
- **Algoritmo de Score**: `(frequência * 10) / dias_ultima_compra`
- **Classificação ABC**: Segmentação automática via NTILE
- **Cross-selling**: Produtos relacionados por correlação
- **Benchmark da rede**: Comparação vs média do grupo

### 🔐 **Segurança & Performance**
- Autenticação JWT robusta (24h de validade)
- Logs detalhados para auditoria
- Pool de conexões otimizado
- Executável compilado com otimizações máximas (~6.3MB)

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
- `GET /health` - Status da API e conexões

### Autenticação
- `POST /auth/login` - Autenticação JWT
- `GET /auth/validate` - Validar token

### 🔥 **Cards Analytics (NOVO!)**
- `GET /analytics/recompra-inteligente` - Card 01: Sugestões IA de recompra
- `GET /analytics/oportunidades-rede` - Card 02: Benchmark vs rede

### Dados & Consultas
- `GET /data/vendas` - Consultar vendas FC (PostgreSQL)
- `POST /portal/query` - Query dinâmica Portal (SQL Server)
- `GET /portal/produtos` - Produtos disponíveis
- `POST /protheus/query` - Consultas Protheus ERP

## 🎯 Cards Analytics - Exemplos

### Card 01: Recompra Inteligente
```bash
GET /analytics/recompra-inteligente?cnpj=17.311.174/0001-78&periodo_dias=180&limite=30
```

**Response:**
```json
{
  "produtos_recompra": [
    {
      "codigo_produto": "PA000037",
      "score_recompra": 4.2,
      "nivel_prioridade": "ALTA",
      "sugestao_inteligente": "Produto em reposição! Incluir no próximo pedido.",
      "produtos_relacionados": [...]
    }
  ]
}
```

### Card 02: Oportunidades na Rede
```bash
GET /analytics/oportunidades-rede?cnpj=17.311.174/0001-78&periodo_dias=90&limite=20
```

**Response:**
```json
{
  "oportunidades": [
    {
      "codigo_produto": "PA000025",
      "seu_grupo": "A",
      "diferenca_percentual": -55.6,
      "oportunidade_reais": 2400.00,
      "insight": "GRANDE OPORTUNIDADE: Você está 55% abaixo da média!",
      "recomendacao": "INCLUIR NO PRÓXIMO PEDIDO"
    }
  ]
}
```

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
- ✅ **Autenticação**: JWT implementado e funcional
- ✅ **Integração tripla**: PostgreSQL + SQL Server Portal + Protheus
- ✅ **🔥 Cards Analytics**: Implementados e testados
  - ✅ Card 01: Algoritmo de recompra inteligente
  - ✅ Card 02: Análise comparativa vs rede
- ✅ **Collection Postman**: Completa com testes automatizados
- ✅ **Documentação**: Guias e exemplos detalhados
- ✅ **Deploy Ready**: Scripts de produção preparados

## 🤝 Contribuição

Este é um projeto interno da Artesanal Farmácia.

## 📄 Licença

Proprietário - Artesanal Farmácia

---

**Desenvolvido com ❤️ e ⚡ em Rust**
