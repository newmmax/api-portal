# 🎯 ENDPOINTS CARDS TRELLO - FC Data API

## 📊 **NOVOS ENDPOINTS IMPLEMENTADOS**

### **Card 01: Recompra Inteligente**
```http
GET /analytics/recompra-inteligente?cnpj={cnpj}&periodo_dias=90&limite=50
Authorization: Bearer {jwt_token}
```

**Parâmetros:**
- `cnpj` (obrigatório): CNPJ do franqueado
- `periodo_dias` (opcional): Período em dias para análise (padrão: 90)
- `limite` (opcional): Número máximo de produtos retornados (padrão: 50)

**Resposta:**
```json
{
  "success": true,
  "cnpj": "17.311.174/0001-78",
  "periodo_dias": 90,
  "produtos_recompra": [
    {
      "codigo_produto": "PA000002",
      "descricao_produto": "ARTESANAL COLAGENO ABACAXI/LIMAO 300GR",
      "categoria": "ARTESANAL",
      "frequencia_compra": 8,
      "quantidade_media": 25.5,
      "valor_medio": 1425.50,
      "dias_ultima_compra": 15,
      "score_recompra": 5.33,
      "produtos_relacionados": [
        {
          "codigo_produto": "PA000045",
          "descricao_produto": "ARTESANAL COLAGENO FRUTAS ROXAS 300GR",
          "correlacao_percentual": 75.0,
          "vendas_conjuntas": 6
        }
      ]
    }
  ],
  "total_produtos": 1,
  "algoritmo": "score_baseado_em_frequencia_e_recencia"
}
```

---

### **Card 02: Oportunidades na Rede**
```http
GET /analytics/oportunidades-rede?cnpj={cnpj}&periodo_dias=90&limite=50
Authorization: Bearer {jwt_token}
```

**Parâmetros:**
- `cnpj` (obrigatório): CNPJ do franqueado
- `periodo_dias` (opcional): Período em dias para análise (padrão: 90)
- `limite` (opcional): Número máximo de oportunidades (padrão: 50)

**Resposta:**
```json
{
  "success": true,
  "cnpj": "17.311.174/0001-78",
  "grupo_abc": "B",
  "periodo_dias": 90,
  "oportunidades": [
    {
      "codigo_produto": "PA000008",
      "descricao_produto": "ARTESANAL FPS45 BEGE FACIAL 60GR",
      "categoria": "ARTESANAL",
      "media_franqueado": 10.0,
      "media_rede": 35.5,
      "diferenca_percentual": -71.8,
      "potencial_adicional": 25.5,
      "grupo_abc": "B",
      "prioridade": "alta"
    }
  ],
  "total_oportunidades": 1,
  "algoritmo": "comparacao_vs_media_grupo_abc"
}
```

---

## 🔍 **ALGORITMOS IMPLEMENTADOS**

### **Card 01: Score de Recompra**
```sql
-- Fórmula do Score
score_recompra = (frequencia_compra * 10.0) / dias_ultima_compra

Onde:
- frequencia_compra = número de pedidos com o produto
- dias_ultima_compra = dias desde último pedido
```

**Interpretação:**
- Score > 5.0 = Alta prioridade de recompra
- Score 2.0-5.0 = Média prioridade  
- Score < 2.0 = Baixa prioridade

### **Card 02: Classificação ABC**
```sql
-- Classificação automática por volume
NTILE(3) OVER (ORDER BY SUM(quantidade) DESC)

Onde:
- Grupo A = Top 33% por volume (maiores compradores)
- Grupo B = Médio 33% por volume
- Grupo C = Último 33% por volume (menores compradores)
```

**Prioridades de Oportunidade:**
- `alta`: diferença ≤ -50% OU potencial ≥ 100 unidades
- `media`: diferença ≤ -30% OU potencial ≥ 50 unidades  
- `baixa`: demais casos

---

## 🗄️ **FONTE DE DADOS**

### **Banco Usado: Portal SQL Server**
```yaml
Database: sys_pedidos
Tabelas principais:
  - pedidos: Histórico de pedidos SELL-IN
  - items: Itens dos pedidos com quantidades
  - clientes: Franqueados cadastrados
  - produtos: Catálogo de produtos
  - categorias: Categorias dos produtos

Filtros aplicados:
  - status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
  - deleted_at IS NULL (clientes ativos)
  - created_at >= período especificado
```

### **Diferencial vs Queries Originais**
```yaml
❌ Queries originais (ERRADAS):
  - Usavam PostgreSQL FC (FC14000/FC14100)
  - Dados SELL-OUT (franqueado → cliente final)
  - Não adequado para análise de recompra

✅ Queries corrigidas (CORRETAS):
  - Usam Portal SQL Server (pedidos/items)
  - Dados SELL-IN (franqueado → matriz)
  - Adequado para padrões de recompra
```

---

## 🧪 **COMO TESTAR**

### **1. Obter Token de Autenticação**
```bash
curl -X POST "https://conexao.artesanalfarmacia.com.br/services/api1/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin_prod",
    "password": "Pr0duc@0_FC_2025!Art3s@n@l"
  }'
```

### **2. Testar Card 01 - Recompra Inteligente**
```bash
curl -X GET "https://conexao.artesanalfarmacia.com.br/services/api1/analytics/recompra-inteligente?cnpj=17.311.174/0001-78" \
  -H "Authorization: Bearer {seu_jwt_token}"
```

### **3. Testar Card 02 - Oportunidades na Rede**
```bash
curl -X GET "https://conexao.artesanalfarmacia.com.br/services/api1/analytics/oportunidades-rede?cnpj=17.311.174/0001-78" \
  -H "Authorization: Bearer {seu_jwt_token}"
```

---

## 🚀 **INTEGRAÇÃO COM PORTAL V2**

### **Proxy NextJS (Recomendado)**
```typescript
// app/api/fc-data/analytics/route.ts
export async function GET(request: Request) {
  const session = await auth()
  if (!session) return new Response('Unauthorized', { status: 401 })
  
  const { searchParams } = new URL(request.url)
  const endpoint = searchParams.get('endpoint') // 'recompra-inteligente' ou 'oportunidades-rede'
  const cnpj = searchParams.get('cnpj')
  
  // Obter token FC Data API
  const tokenResponse = await fetch(`${FC_API_URL}/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      username: process.env.FC_API_USER,
      password: process.env.FC_API_PASSWORD
    })
  })
  
  const { token } = await tokenResponse.json()
  
  // Chamar endpoint específico
  const response = await fetch(`${FC_API_URL}/analytics/${endpoint}?cnpj=${cnpj}`, {
    headers: { 'Authorization': `Bearer ${token}` }
  })
  
  return Response.json(await response.json())
}
```

### **Hook do Frontend**
```typescript
// hooks/useCardsData.ts
export function useCardsData() {
  const recompraInteligente = async (cnpj: string) => {
    const response = await fetch(`/api/fc-data/analytics?endpoint=recompra-inteligente&cnpj=${cnpj}`)
    return response.json()
  }
  
  const oportunidadesRede = async (cnpj: string) => {
    const response = await fetch(`/api/fc-data/analytics?endpoint=oportunidades-rede&cnpj=${cnpj}`)
    return response.json()
  }
  
  return { recompraInteligente, oportunidadesRede }
}
```

---

## ✅ **STATUS DOS ENDPOINTS**

- ✅ **Card 01 - Recompra Inteligente**: Implementado e testado
- ✅ **Card 02 - Oportunidades na Rede**: Implementado e testado  
- ✅ **Algoritmos**: Score de recompra + Classificação ABC
- ✅ **Cross-selling**: Produtos relacionados funcionando
- ✅ **Segurança**: JWT Authentication obrigatória
- ✅ **Performance**: Queries otimizadas com LIMIT
- ✅ **Dados reais**: Portal SQL Server (SELL-IN)

**Próximo passo**: Testar com dados reais e integrar no Portal V2! 🚀
