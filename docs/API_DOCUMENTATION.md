# 📚 FC DATA API - DOCUMENTAÇÃO COMPLETA

## 📋 Índice
1. [Visão Geral](#visão-geral)
2. [Instalação e Configuração](#instalação-e-configuração)
3. [Autenticação](#autenticação)
4. [Endpoints](#endpoints)
5. [Modelos de Dados](#modelos-de-dados)
6. [Códigos de Erro](#códigos-de-erro)
7. [Exemplos Práticos](#exemplos-práticos)
8. [Deploy](#deploy)

## 🎯 Visão Geral

A FC Data API é uma API RESTful unificada que conecta três bancos de dados:
- **PostgreSQL**: Dados de vendas (Formula Certa)
- **SQL Server Portal**: Sistema de pedidos
- **SQL Server Protheus**: ERP TOTVS

### Características Principais
- ✅ Autenticação JWT
- ✅ Multi-database
- ✅ CRUD completo de pedidos
- ✅ Queries customizadas
- ✅ Validações de negócio
- ✅ Transações ACID

### URL Base
- **Desenvolvimento**: `http://localhost:8089/services/api1`
- **Produção**: `https://seu-dominio.com/services/api1`

## 🔧 Instalação e Configuração

### Requisitos
- Rust 1.70+
- PostgreSQL
- SQL Server
- Windows/Linux

### Instalação

```bash
# Clone o repositório
cd C:\XAMPP\htdocs\portaldepedidos\fc-data-api

# Instale as dependências
cargo build --release

# Configure o .env (veja exemplo abaixo)
copy .env.example .env

# Execute
cargo run
```

### Configuração (.env)

```env
# Servidor
SERVER_HOST=127.0.0.1
SERVER_PORT=8089

# PostgreSQL - Formula Certa
DATABASE_URL=postgres://usuario:senha@host:5432/fc_data

# SQL Server - Portal de Pedidos
PORTAL_DATABASE_HOST=10.216.1.11
PORTAL_DATABASE_PORT=1433
PORTAL_DATABASE_NAME=sys_pedidos
PORTAL_DATABASE_USER=sa
PORTAL_DATABASE_PASS=senha
PORTAL_CONNECTION_STRING=Server=tcp:10.216.1.11,1433;Database=sys_pedidos;UID=sa;PWD=senha;TrustServerCertificate=true

# SQL Server - Protheus ERP
PROTHEUS_DATABASE_HOST=10.216.1.11
PROTHEUS_DATABASE_PORT=1433
PROTHEUS_DATABASE_NAME=sigaofc
PROTHEUS_DATABASE_USER=sa
PROTHEUS_DATABASE_PASS=senha
PROTHEUS_CONNECTION_STRING=Server=tcp:10.216.1.11,1433;Database=sigaofc;UID=sa;PWD=senha;TrustServerCertificate=true

# JWT
JWT_SECRET=seu_secret_key_seguro
JWT_EXPIRATION_HOURS=24

# CORS
CORS_ALLOWED_ORIGINS=http://localhost:3000,https://app.exemplo.com

# Admin
ADMIN_USERNAME=admin
ADMIN_PASSWORD=senha_forte

# Logs
RUST_LOG=info,fc_data_api=debug

# API
API_PREFIX=/services/api1
```

## 🔐 Autenticação

### Login
```http
POST /auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "ArtesanalFC2025!"
}
```

**Response:**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "expires_in": 86400
}
```

### Validar Token
```http
GET /auth/validate
Authorization: Bearer {token}
```

**Response:**
```json
{
  "valid": true,
  "username": "admin"
}
```

## 📡 Endpoints

### Health Check

#### Verificar Status da API
```http
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "message": "FC Data API Unificada está operacional",
  "timestamp": "2025-01-11T12:00:00Z",
  "databases": {
    "postgres_fc": {
      "status": "conectado",
      "database": "fc_data"
    },
    "portal_pedidos": {
      "status": "conectado",
      "database": "sys_pedidos"
    },
    "protheus": {
      "status": "conectado",
      "database": "sigaofc"
    }
  }
}
```

### Formula Certa (PostgreSQL)

#### Buscar Vendas
```http
GET /data/vendas?data_inicio=2025-01-01&data_fim=2025-01-31&empresa=1
Authorization: Bearer {token}
```

**Query Parameters:**
- `data_inicio` (required): Data inicial (YYYY-MM-DD)
- `data_fim` (required): Data final (YYYY-MM-DD)
- `empresa` (optional): ID da empresa
- `vendedor` (optional): Código do vendedor
- `cliente` (optional): Código do cliente
- `produto` (optional): Código do produto

#### Query Customizada PostgreSQL
```http
POST /data/query
Authorization: Bearer {token}
Content-Type: application/json

{
  "query": "SELECT * FROM fc14000 WHERE company_id = $1",
  "params": [1]
}
```

### Portal de Pedidos (SQL Server)

#### Listar Produtos
```http
GET /portal/produtos?cliente_id=5&limite=10&apenas_ativos=true
Authorization: Bearer {token}
```

**Query Parameters:**
- `cliente_id` (optional): ID do cliente para buscar preços
- `grupo_venda` (optional): Grupo de venda específico
- `apenas_ativos` (optional): true/false (default: true)
- `limite` (optional): Número máximo de resultados (default: 100)

#### Query Customizada Portal
```http
POST /portal/query
Authorization: Bearer {token}
Content-Type: application/json

{
  "sql": "SELECT * FROM pedidos WHERE cliente_id = @P1",
  "params": [123]
}
```

### CRUD de Pedidos

#### Criar Pedido
```http
POST /portal/pedidos
Authorization: Bearer {token}
Content-Type: application/json

{
  "codigo_cliente": "000005",
  "loja_cliente": "01",
  "emissao": "2025-01-11",
  "natureza": "10212",
  "mensagem": "Observações do pedido",
  "regra_condicao_pagamento_id": 1,
  "regra_frete_id": 1,
  "items": [
    {
      "produto_id": 123,
      "quantidade": 10
    }
  ]
}
```

#### Buscar Pedido
```http
GET /portal/pedidos/{id}
Authorization: Bearer {token}
```

#### Atualizar Pedido
```http
PUT /portal/pedidos/{id}
Authorization: Bearer {token}
Content-Type: application/json

{
  "codigo_cliente": "000005",
  "loja_cliente": "01",
  "emissao": "2025-01-11",
  "natureza": "10212",
  "mensagem": "Pedido atualizado",
  "regra_condicao_pagamento_id": 2,
  "regra_frete_id": 1,
  "items": [
    {
      "produto_id": 123,
      "quantidade": 20
    }
  ]
}
```

#### Deletar Pedido
```http
DELETE /portal/pedidos/{id}
Authorization: Bearer {token}
```

#### Confirmar Pedido
```http
POST /portal/pedidos/{id}/confirmar
Authorization: Bearer {token}
```

### Protheus ERP (SQL Server)

#### Query Customizada Protheus
```http
POST /protheus/query
Authorization: Bearer {token}
Content-Type: application/json

{
  "sql": "SELECT * FROM ZC7010 WHERE ZC7_NUM = @P1",
  "params": ["000123"]
}
```

#### Status do Pedido no Protheus (Em Desenvolvimento)
```http
GET /protheus/pedidos/{numero}/status
Authorization: Bearer {token}
```

### Analytics (Em Desenvolvimento)

#### Analytics 360° do Cliente
```http
GET /analytics/cliente/{cnpj}/360?periodo=30d
Authorization: Bearer {token}
```

#### Correlações de Produto
```http
GET /analytics/produtos/{id}/correlacoes
Authorization: Bearer {token}
```

## 📊 Modelos de Dados

### Pedido
```typescript
interface Pedido {
  id: number;
  cliente_id: number;
  codigo_cliente: string;
  numero_pedido?: string;
  loja_cliente: string;
  emissao: string;
  mensagem?: string;
  natureza: string;
  status_pedido: "a confirmar" | "confirmado" | "integrado";
  regra_condicao_pagamento_id: number;
  regra_frete_id: number;
  created_at?: string;
  updated_at?: string;
}
```

### Item do Pedido
```typescript
interface ItemPedido {
  id?: number;
  pedido_id?: number;
  produto_id: number;
  quantidade: number;
  preco_unitario?: number;
}
```

### Produto
```typescript
interface Produto {
  id: number;
  codigo: string;
  descricao: string;
  unidade_medida: string;
  quantidade_minima_embalagem: number;
  saldo: number;
  preco?: number;
  status: boolean;
  grupo_venda?: string;
}
```

## ❌ Códigos de Erro

| Código | Status | Descrição |
|--------|--------|-----------|
| 400 | Bad Request | Dados inválidos ou faltando |
| 401 | Unauthorized | Token inválido ou expirado |
| 403 | Forbidden | Sem permissão para a operação |
| 404 | Not Found | Recurso não encontrado |
| 500 | Internal Server Error | Erro interno do servidor |

### Formato de Erro
```json
{
  "error": true,
  "message": "Descrição do erro",
  "code": 400
}
```

## 🚀 Exemplos Práticos

### Exemplo Completo com cURL

```bash
# 1. Login
TOKEN=$(curl -s -X POST http://localhost:8089/services/api1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "ArtesanalFC2025!"}' \
  | jq -r '.token')

# 2. Criar Pedido
PEDIDO_ID=$(curl -s -X POST http://localhost:8089/services/api1/portal/pedidos \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "codigo_cliente": "000005",
    "loja_cliente": "01",
    "emissao": "2025-01-11",
    "natureza": "10212",
    "mensagem": "Teste via API",
    "regra_condicao_pagamento_id": 1,
    "regra_frete_id": 1,
    "items": [
      {"produto_id": 123, "quantidade": 10}
    ]
  }' | jq -r '.pedido_id')

# 3. Confirmar Pedido
curl -X POST http://localhost:8089/services/api1/portal/pedidos/$PEDIDO_ID/confirmar \
  -H "Authorization: Bearer $TOKEN"
```

### Exemplo JavaScript/TypeScript

```typescript
// Cliente API
class FCDataAPI {
  private baseURL = 'http://localhost:8089/services/api1';
  private token: string = '';

  async login(username: string, password: string) {
    const response = await fetch(`${this.baseURL}/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password })
    });
    const data = await response.json();
    this.token = data.token;
    return data;
  }

  async criarPedido(pedido: any) {
    const response = await fetch(`${this.baseURL}/portal/pedidos`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${this.token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(pedido)
    });
    return response.json();
  }
}

// Uso
const api = new FCDataAPI();
await api.login('admin', 'ArtesanalFC2025!');
const pedido = await api.criarPedido({
  codigo_cliente: "000005",
  loja_cliente: "01",
  // ... resto dos dados
});
```

### Exemplo Python

```python
import requests
import json

class FCDataAPI:
    def __init__(self, base_url='http://localhost:8089/services/api1'):
        self.base_url = base_url
        self.token = None
    
    def login(self, username, password):
        response = requests.post(
            f'{self.base_url}/auth/login',
            json={'username': username, 'password': password}
        )
        data = response.json()
        self.token = data['token']
        return data
    
    def criar_pedido(self, pedido):
        response = requests.post(
            f'{self.base_url}/portal/pedidos',
            headers={'Authorization': f'Bearer {self.token}'},
            json=pedido
        )
        return response.json()

# Uso
api = FCDataAPI()
api.login('admin', 'ArtesanalFC2025!')
pedido = api.criar_pedido({
    'codigo_cliente': '000005',
    'loja_cliente': '01',
    # ... resto dos dados
})
```

## 🌐 CORS

A API está configurada para aceitar requisições das origens definidas em `CORS_ALLOWED_ORIGINS`.

Headers CORS enviados:
- `Access-Control-Allow-Origin`
- `Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS`
- `Access-Control-Allow-Headers: Content-Type, Authorization`

## 🔒 Segurança

1. **JWT**: Tokens expiram em 24 horas (configurável)
2. **HTTPS**: Use sempre HTTPS em produção
3. **Rate Limiting**: Implemente no proxy reverso
4. **Validações**: Todos os inputs são validados
5. **SQL Injection**: Usa prepared statements

## 📈 Performance

- Pool de conexões para cada banco
- 20 workers por padrão (configurável)
- Queries otimizadas com índices
- Suporta milhares de requisições/segundo

## 🧪 Testes

### Bases de Teste
Para desenvolvimento, use:
- `sys_pedidos_teste` (Portal)
- `sigaofc_teste` (Protheus)

### Health Check Rápido
```bash
curl http://localhost:8089/services/api1/health
```

## 🔧 Troubleshooting

### Erro de Conexão
- Verifique as credenciais no .env
- Confirme que os bancos estão acessíveis
- Teste conectividade com telnet

### Token Inválido
- Verifique se não expirou (24h)
- Confirme o JWT_SECRET no .env

### Porta em Uso
```bash
# Windows
netstat -ano | findstr :8089
taskkill /F /PID {PID}

# Linux
lsof -i :8089
kill -9 {PID}
```

## 📞 Suporte

Para suporte e dúvidas:
- Documentação: `/docs/`
- Logs: `RUST_LOG=debug cargo run`
