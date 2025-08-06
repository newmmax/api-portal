# 🚀 FC Data API - API Unificada Multi-Database

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Status](https://img.shields.io/badge/status-production%20ready-green)

## 📋 Sumário

- [Visão Geral](#-visão-geral)
- [Arquitetura](#-arquitetura)
- [Funcionalidades](#-funcionalidades)
- [Instalação](#-instalação)
- [Configuração](#-configuração)
- [Uso](#-uso)
- [API Reference](#-api-reference)
- [Deploy](#-deploy)
- [Contribuindo](#-contribuindo)

## 🎯 Visão Geral

A **FC Data API** é uma API RESTful desenvolvida em Rust que unifica o acesso a três bancos de dados diferentes, fornecendo uma interface única e segura para operações de dados e gerenciamento de pedidos.

### Por que esta API?

- **Unificação**: Acesso a 3 bancos diferentes com uma única autenticação
- **Segurança**: JWT, validações e controle de acesso
- **Performance**: Rust + pools de conexão otimizados
- **Flexibilidade**: Queries customizadas e endpoints específicos
- **Confiabilidade**: Transações ACID e tratamento de erros robusto

## 🏗️ Arquitetura

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   PostgreSQL    │     │   SQL Server     │     │   SQL Server    │
│   (FC Data)     │     │   (Portal)       │     │   (Protheus)    │
└────────┬────────┘     └────────┬─────────┘     └────────┬─────────┘
         │                       │                          │
         └───────────────────────┴──────────────────────────┘
                                 │
                        ┌────────▼─────────┐
                        │   FC DATA API    │
                        │   (Rust/Actix)   │
                        └────────┬─────────┘
                                 │ JWT Auth
                        ┌────────▼─────────┐
                        │  Client Apps     │
                        │  (Web/Mobile)    │
                        └──────────────────┘
```

### Stack Tecnológico

- **Linguagem**: Rust 1.70+
- **Framework**: Actix-web 4.x
- **Bancos de Dados**: 
  - PostgreSQL (Formula Certa)
  - SQL Server (Portal de Pedidos)
  - SQL Server (Protheus ERP)
- **Autenticação**: JWT
- **Serialização**: Serde
- **Logs**: env_logger

## ✨ Funcionalidades

### Autenticação e Segurança
- ✅ Login com JWT
- ✅ Validação de tokens
- ✅ CORS configurável
- ✅ Rate limiting (via proxy)

### Formula Certa (PostgreSQL)
- ✅ Consulta de vendas com filtros
- ✅ Queries customizadas
- ✅ Análise de dados de vendas

### Portal de Pedidos (SQL Server)
- ✅ CRUD completo de pedidos
- ✅ Listagem de produtos com preços por grupo
- ✅ Validações de negócio complexas
- ✅ Confirmação de pedidos
- ✅ Queries customizadas

### Protheus ERP (SQL Server)
- ✅ Queries customizadas
- 🔄 Integração com pedidos (em desenvolvimento)
- 🔄 Status de pedidos (em desenvolvimento)

### Sistema
- ✅ Health check multi-database
- ✅ Logs estruturados
- ✅ Tratamento de erros padronizado
- ✅ Pool de conexões otimizado

## 🔧 Instalação

### Pré-requisitos

- Rust 1.70+ ([instalar](https://rustup.rs/))
- PostgreSQL 12+
- SQL Server 2016+
- Git

### Clone e Build

```bash
# Clone o repositório
git clone https://github.com/sua-empresa/fc-data-api.git
cd fc-data-api

# Instale as dependências e compile
cargo build --release

# Para desenvolvimento
cargo build
```

## ⚙️ Configuração

### 1. Configure o arquivo `.env`

```bash
cp .env.example .env
```

Edite o `.env` com suas configurações:

```env
# Servidor
SERVER_HOST=127.0.0.1
SERVER_PORT=8089

# PostgreSQL
DATABASE_URL=postgres://user:pass@host:5432/fc_data

# SQL Server Portal
PORTAL_CONNECTION_STRING=Server=tcp:host,1433;Database=sys_pedidos;UID=user;PWD=pass;TrustServerCertificate=true

# SQL Server Protheus
PROTHEUS_CONNECTION_STRING=Server=tcp:host,1433;Database=sigaofc;UID=user;PWD=pass;TrustServerCertificate=true

# JWT
JWT_SECRET=your-secret-key-here
JWT_EXPIRATION_HOURS=24

# Admin
ADMIN_USERNAME=admin
ADMIN_PASSWORD=strong-password

# Logs
RUST_LOG=info,fc_data_api=debug
```

### 2. Teste a configuração

```bash
# Execute em modo desenvolvimento
cargo run

# Teste o health check
curl http://localhost:8089/services/api1/health
```

## 🚀 Uso

### Iniciar o servidor

```bash
# Desenvolvimento
cargo run

# Produção
./target/release/fc-data-api
```

### Exemplo de uso básico

```javascript
// 1. Login
const response = await fetch('http://localhost:8089/services/api1/auth/login', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    username: 'admin',
    password: 'your-password'
  })
});
const { token } = await response.json();

// 2. Criar um pedido
const pedido = await fetch('http://localhost:8089/services/api1/portal/pedidos', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    codigo_cliente: '000005',
    loja_cliente: '01',
    emissao: '2025-01-11',
    natureza: '10212',
    mensagem: 'Pedido via API',
    regra_condicao_pagamento_id: 1,
    regra_frete_id: 1,
    items: [
      { produto_id: 123, quantidade: 10 }
    ]
  })
});
```

## 📚 API Reference

### Documentação Completa

Veja a [documentação completa da API](docs/API_DOCUMENTATION.md) para todos os endpoints disponíveis.

### Principais Endpoints

| Método | Endpoint | Descrição |
|--------|----------|-----------|
| POST | `/auth/login` | Autenticação |
| GET | `/health` | Status da API |
| GET | `/data/vendas` | Consultar vendas |
| POST | `/portal/pedidos` | Criar pedido |
| GET | `/portal/pedidos/{id}` | Buscar pedido |
| PUT | `/portal/pedidos/{id}` | Atualizar pedido |
| DELETE | `/portal/pedidos/{id}` | Deletar pedido |
| POST | `/portal/pedidos/{id}/confirmar` | Confirmar pedido |

## 🌐 Deploy

### Opções de Deploy

1. **Windows Service**: Usando NSSM
2. **Linux Systemd**: Serviço nativo
3. **Docker**: Container (em desenvolvimento)
4. **Cloud**: AWS, Azure, GCP

Veja o [guia completo de deploy](docs/DEPLOY_GUIDE.md) para instruções detalhadas.

### Deploy Rápido (Linux)

```bash
# Build para produção
cargo build --release

# Copie o executável
sudo cp target/release/fc-data-api /opt/fc-data-api/

# Configure como serviço
sudo systemctl enable fc-data-api
sudo systemctl start fc-data-api
```

## 🔍 Sistema de Logs & Debug

### Sistema de Logs Automático

A API inclui um sistema avançado de logs específico para Cards Analytics que permite debug remoto sem acesso ao servidor.

#### Configuração
```bash
# No arquivo .env (desenvolvimento)
ENABLE_DEBUG_LOGS=true
DEBUG_LOG_FILE=D:\PROJETOS\RUST\fc-data-api\cards_debug.log

# No arquivo .env.production (produção)
ENABLE_DEBUG_LOGS=true
DEBUG_LOG_FILE=C:\Service\logs\cards_debug.log
```

#### Endpoints de Debug (Requerem JWT)
| Método | Endpoint | Descrição |
|--------|----------|-----------|
| GET | `/debug/logs/status` | Status do sistema de logs |
| GET | `/debug/logs` | Visualizar logs com filtros |
| POST | `/debug/logs/rotate` | Limpar logs antigos |

#### Filtros Disponíveis
```bash
# Apenas erros
GET /debug/logs?nivel=ERROR&linhas=30

# CNPJ específico
GET /debug/logs?cnpj=17311174000178&linhas=40

# Card específico
GET /debug/logs?card=oportunidades-rede&linhas=20

# Por tempo
GET /debug/logs?desde=2025-08-06T10:00:00Z
```

#### Setup Automático (Produção)
```bash
# Execute na pasta do projeto
./setup_logs_producao.bat

# Verifica/cria C:\Service\logs\
# Configura permissões
# Valida configuração
```

**📋 Veja detalhes completos**: [SISTEMA_LOGS_PRODUCAO.md](SISTEMA_LOGS_PRODUCAO.md)

## 🧪 Testes

### Executar testes

```bash
# Testes unitários
cargo test

# Testes de integração
cargo test --test '*' -- --test-threads=1

# Com cobertura
cargo tarpaulin --out Html
```

### Bases de teste

Use as bases de teste para desenvolvimento:
- `sys_pedidos_teste`
- `sigaofc_teste`

## 📊 Performance

- **Requisições/segundo**: 5000+ (hardware médio)
- **Latência média**: < 50ms
- **Uso de memória**: ~50MB
- **Workers**: 20 (configurável)

## 🤝 Contribuindo

1. Fork o projeto
2. Crie sua feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

### Padrões de código

- Use `cargo fmt` antes de commitar
- Execute `cargo clippy` para linting
- Adicione testes para novas funcionalidades
- Documente funções públicas

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

## 🆘 Suporte

- **Documentação**: [/docs](./docs)
- **Issues**: [GitHub Issues](https://github.com/sua-empresa/fc-data-api/issues)
- **Email**: suporte@suaempresa.com.br

## 🏆 Agradecimentos

- Time de desenvolvimento da Artesanal Farmácia
- Comunidade Rust
- Contribuidores do projeto

---

Desenvolvido com ❤️ em Rust
