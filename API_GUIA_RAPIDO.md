# 🚀 FC Data API - Guia Rápido

## URL de Produção
```
https://conexao.artesanalfarmacia.com.br/services/api1
```

## 🔑 Credenciais
```
Username: admin_prod
Password: Pr0duc@0_FC_2025!Art3s@n@l
```

## 📡 Endpoints Principais

### 1. Health Check (Público)
```bash
GET https://conexao.artesanalfarmacia.com.br/services/api1/health
```

### 2. Login
```bash
POST https://conexao.artesanalfarmacia.com.br/services/api1/auth/login
Content-Type: application/json

{
  "username": "admin_prod",
  "password": "Pr0duc@0_FC_2025!Art3s@n@l"
}
```

### 3. Consultar Vendas
```bash
GET https://conexao.artesanalfarmacia.com.br/services/api1/data/vendas?data_inicio=2025-07-01&data_fim=2025-07-31&limite=50
Authorization: Bearer SEU_TOKEN_AQUI
```

## 💻 Exemplo JavaScript Rápido

```javascript
// 1. Login
const loginResponse = await fetch('https://conexao.artesanalfarmacia.com.br/services/api1/auth/login', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    username: 'admin_prod',
    password: 'Pr0duc@0_FC_2025!Art3s@n@l'
  })
});
const { token } = await loginResponse.json();

// 2. Buscar vendas
const vendasResponse = await fetch('https://conexao.artesanalfarmacia.com.br/services/api1/data/vendas?limite=10', {
  headers: { 'Authorization': `Bearer ${token}` }
});
const vendas = await vendasResponse.json();

console.log(vendas);
```

## 📋 Parâmetros de Filtro para Vendas

- `data_inicio`: Data inicial (YYYY-MM-DD)
- `data_fim`: Data final (YYYY-MM-DD)
- `limite`: Número máximo de registros
- `empresa`: Nome da empresa (opcional)
- `filial`: Código da filial (opcional)
- `vendedor`: Código do vendedor (opcional)
- `produto`: Nome do produto - busca parcial (opcional)

## ⚡ Teste Rápido com cURL

```bash
# Health Check
curl https://conexao.artesanalfarmacia.com.br/services/api1/health

# Login e pegar token
curl -X POST https://conexao.artesanalfarmacia.com.br/services/api1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin_prod","password":"Pr0duc@0_FC_2025!Art3s@n@l"}'
```

---
**Token expira em:** 24 horas  
**Documentação completa:** DOCUMENTACAO_API_PRODUCAO.md