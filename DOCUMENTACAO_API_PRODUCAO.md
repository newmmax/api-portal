# 📚 FC Data API - Documentação de Produção

## 🌐 Informações de Acesso

**URL Base de Produção:**
```
https://conexao.artesanalfarmacia.com.br/services/api1
```

**Status da API:**
- ✅ Online e operacional
- ✅ Todos os bancos de dados conectados
- ✅ Acessível via HTTPS

---

## 🔐 Autenticação

A API utiliza autenticação JWT. Você precisa fazer login para obter um token que deve ser enviado em todas as requisições.

### Login

**Endpoint:** `POST /auth/login`

**Request:**
```bash
curl -X POST https://conexao.artesanalfarmacia.com.br/services/api1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin_prod",
    "password": "Pr0duc@0_FC_2025!Art3s@n@l"
  }'
```

**Response:**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

**⚠️ IMPORTANTE:** O token expira em 24 horas (86400 segundos).

### Validar Token

**Endpoint:** `GET /auth/validate`

**Request:**
```bash
curl https://conexao.artesanalfarmacia.com.br/services/api1/auth/validate \
  -H "Authorization: Bearer SEU_TOKEN_AQUI"
```

---

## 📊 Endpoints de Consulta

### 1. Health Check (Público)

Verifica o status da API e conexões com bancos de dados.

**Endpoint:** `GET /health`

**Request:**
```bash
curl https://conexao.artesanalfarmacia.com.br/services/api1/health
```

**Response:**
```json
{
  "status": "healthy",
  "message": "FC Data API Unificada está operacional",
  "databases": {
    "postgres_fc": {
      "database": "fc_data",
      "status": "conectado"
    },
    "portal_pedidos": {
      "database": "sys_pedidos",
      "status": "conectado"
    },
    "protheus": {
      "database": "SIGAOFC",
      "status": "conectado"
    }
  },
  "timestamp": "2025-07-13T23:52:37.595619300+00:00"
}
```

### 2. Consultar Vendas

Retorna dados detalhados de vendas do sistema FC.

**Endpoint:** `GET /data/vendas`

**Headers Obrigatórios:**
- `Authorization: Bearer SEU_TOKEN_AQUI`

**Parâmetros de Query:**
| Parâmetro | Tipo | Descrição | Exemplo |
|-----------|------|-----------|---------|
| `data_inicio` | string | Data inicial (YYYY-MM-DD) | 2025-01-01 |
| `data_fim` | string | Data final (YYYY-MM-DD) | 2025-12-31 |
| `limite` | number | Número máximo de registros | 100 |
| `empresa` | string | Filtrar por empresa (opcional) | MATRIZ |
| `filial` | string | Código da filial (opcional) | 1 |
| `vendedor` | string | Código do vendedor (opcional) | 10 |
| `produto` | string | Nome do produto - busca parcial (opcional) | DIPIRONA |

**Request Exemplo:**
```bash
curl "https://conexao.artesanalfarmacia.com.br/services/api1/data/vendas?data_inicio=2025-01-01&data_fim=2025-12-31&limite=10" \
  -H "Authorization: Bearer SEU_TOKEN_AQUI"
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "companygroupname": "GRUPO ARTESANAL",
      "cnpj": "12.345.678/0001-90",
      "cdfil": 1,
      "descrfil": "MATRIZ",
      "nrcpm": 12345,
      "dtpagefe": "2025-01-15",
      "dteminfce": "2025-01-15",
      "cdcli": 1001,
      "nomecli": "JOÃO DA SILVA",
      "cdfunre": 10,
      "nomefun": "MARIA VENDEDORA",
      "itemid": 1,
      "cdpro": "MED001",
      "descrprd": "DIPIRONA 500MG",
      "setor": "MEDICAMENTOS",
      "quant": 2.0,
      "pruni": 5.50,
      "vrtot": 11.00,
      "vrdsc": 0.00,
      "vrrcb": 11.00,
      "prcusto": 3.00,
      "prcompra": 3.50
    }
  ],
  "total": 1,
  "query_info": {
    "data_inicio": "2025-01-01",
    "data_fim": "2025-12-31",
    "limite": 10
  }
}
```

### 3. Query Customizada

Executa queries SQL customizadas (apenas SELECT).

**Endpoint:** `POST /data/query`

**Headers Obrigatórios:**
- `Authorization: Bearer SEU_TOKEN_AQUI`
- `Content-Type: application/json`

**Request:**
```bash
curl -X POST https://conexao.artesanalfarmacia.com.br/services/api1/data/query \
  -H "Authorization: Bearer SEU_TOKEN_AQUI" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "SELECT COUNT(*) as total, SUM(vrrcb) as valor_total FROM fc14100 WHERE cdpro IS NOT NULL LIMIT 10",
    "params": []
  }'
```

---

## 🔧 Exemplos Práticos

### Exemplo 1: Fluxo Completo - Login e Consulta

```bash
# 1. Fazer login e capturar o token
TOKEN=$(curl -s -X POST https://conexao.artesanalfarmacia.com.br/services/api1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin_prod","password":"Pr0duc@0_FC_2025!Art3s@n@l"}' \
  | grep -o '"token":"[^"]*' | cut -d'"' -f4)

# 2. Consultar vendas do mês atual
curl "https://conexao.artesanalfarmacia.com.br/services/api1/data/vendas?data_inicio=2025-07-01&data_fim=2025-07-31&limite=50" \
  -H "Authorization: Bearer $TOKEN"
```

### Exemplo 2: JavaScript/Frontend

```javascript
// Função de login
async function login() {
  const response = await fetch('https://conexao.artesanalfarmacia.com.br/services/api1/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      username: 'admin_prod',
      password: 'Pr0duc@0_FC_2025!Art3s@n@l'
    })
  });
  
  const data = await response.json();
  return data.token;
}

// Função para buscar vendas
async function buscarVendas(token, dataInicio, dataFim) {
  const response = await fetch(
    `https://conexao.artesanalfarmacia.com.br/services/api1/data/vendas?data_inicio=${dataInicio}&data_fim=${dataFim}&limite=100`,
    {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    }
  );
  
  return await response.json();
}

// Uso
(async () => {
  try {
    const token = await login();
    const vendas = await buscarVendas(token, '2025-07-01', '2025-07-31');
    console.log('Vendas:', vendas);
  } catch (error) {
    console.error('Erro:', error);
  }
})();
```

### Exemplo 3: Python

```python
import requests
import json

# URL base da API
BASE_URL = "https://conexao.artesanalfarmacia.com.br/services/api1"

# Login
def login(username, password):
    response = requests.post(
        f"{BASE_URL}/auth/login",
        json={"username": username, "password": password}
    )
    return response.json()["token"]

# Buscar vendas
def buscar_vendas(token, data_inicio, data_fim, limite=100):
    headers = {"Authorization": f"Bearer {token}"}
    params = {
        "data_inicio": data_inicio,
        "data_fim": data_fim,
        "limite": limite
    }
    
    response = requests.get(
        f"{BASE_URL}/data/vendas",
        headers=headers,
        params=params
    )
    return response.json()

# Uso
if __name__ == "__main__":
    # Fazer login
    token = login("admin_prod", "Pr0duc@0_FC_2025!Art3s@n@l")
    
    # Buscar vendas
    vendas = buscar_vendas(token, "2025-07-01", "2025-07-31")
    
    # Processar resultados
    if vendas["success"]:
        print(f"Total de registros: {vendas['total']}")
        for venda in vendas["data"]:
            print(f"Produto: {venda['descrprd']} - Valor: R$ {venda['vrrcb']}")
```

---

## 📋 Campos Retornados nas Vendas

| Campo | Descrição | Tipo |
|-------|-----------|------|
| `companygroupname` | Nome do grupo/empresa | string |
| `cnpj` | CNPJ da empresa | string |
| `cdfil` | Código da filial | number |
| `descrfil` | Descrição da filial | string |
| `nrcpm` | Número do cupom/venda | number |
| `dtpagefe` | Data de pagamento | string (YYYY-MM-DD) |
| `dteminfce` | Data de emissão | string (YYYY-MM-DD) |
| `cdcli` | Código do cliente | number |
| `nomecli` | Nome do cliente | string |
| `cdfunre` | Código do vendedor | number |
| `nomefun` | Nome do vendedor | string |
| `itemid` | ID do item | number |
| `cdpro` | Código do produto | string |
| `descrprd` | Descrição do produto | string |
| `setor` | Setor do produto | string |
| `quant` | Quantidade | number |
| `pruni` | Preço unitário | number |
| `vrtot` | Valor total | number |
| `vrdsc` | Valor de desconto | number |
| `vrrcb` | Valor recebido (calculado) | number |
| `prcusto` | Preço de custo | number |
| `prcompra` | Preço de compra | number |

---

## ⚠️ Limites e Considerações

1. **Limite de Registros:** O parâmetro `limite` tem um máximo recomendado de 1000 registros por consulta
2. **Timeout:** Requisições têm timeout de 300 segundos
3. **Rate Limiting:** Não há limite de requisições implementado atualmente
4. **CORS:** A API aceita requisições apenas de domínios autorizados

---

## 🚨 Códigos de Erro

| Código | Descrição | Solução |
|--------|-----------|---------|
| 401 | Não autorizado | Token inválido ou expirado - faça login novamente |
| 400 | Requisição inválida | Verifique os parâmetros enviados |
| 500 | Erro interno | Verifique os logs ou contate o suporte |

---

## 📞 Suporte

- **Logs do servidor:** `C:\service\logs\service.log`
- **Status do serviço:** `nssm status FCDataAPI`
- **Reiniciar API:** `nssm restart FCDataAPI`

---

## 🔄 Postman Collection

Importe a collection do Postman disponível em:
`FC_Data_API.postman_collection.json`

Configure as variáveis:
- `base_url`: `https://conexao.artesanalfarmacia.com.br/services/api1`
- `token`: (será preenchido automaticamente após login)

---

**Última atualização:** 13/07/2025  
**Versão da API:** 0.1.0