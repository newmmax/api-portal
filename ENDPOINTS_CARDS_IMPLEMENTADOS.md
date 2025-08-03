# 🎯 CARDS ANALYTICS - ENDPOINTS IMPLEMENTADOS

## 📊 **Card 01: Recompra Inteligente**

### Endpoint
```
GET /services/api1/analytics/recompra-inteligente
```

### Parâmetros Query
```yaml
cnpj: "17.311.174/0001-78"          # OBRIGATÓRIO - CNPJ do franqueado
periodo_dias: 180                   # OPCIONAL - Período análise (padrão: 90)
limite: 30                          # OPCIONAL - Top N produtos (padrão: 50)
```

### Headers
```
Authorization: Bearer {JWT_TOKEN}
Content-Type: application/json
```

### Exemplo de Request
```bash
curl -X GET "http://localhost:8089/services/api1/analytics/recompra-inteligente?cnpj=17.311.174/0001-78&periodo_dias=180&limite=30" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
```

### Response Estrutura
```json
{
  "success": true,
  "cnpj": "17.311.174/0001-78",
  "periodo_dias": 180,
  "produtos_recompra": [
    {
      "codigo_produto": "PA000037",
      "descricao_produto": "ARTESANAL FORT CUP 30 CAPS",
      "categoria": "SUPLEMENTOS",
      "frequencia_compra": 4,
      "quantidade_media": 18.0,
      "valor_medio": 450.0,
      "dias_ultima_compra": 15,
      "score_recompra": 4.2,
      "nivel_prioridade": "ALTA",
      "sugestao_inteligente": "Produto em reposição! Sugerimos incluir no próximo pedido.",
      "produtos_relacionados": [
        {
          "codigo_produto": "PA000045",
          "descricao_produto": "ARTESANAL COLÁGENO",
          "correlacao_percentual": 75.0,
          "vendas_conjuntas": 3
        }
      ]
    }
  ],
  "total_produtos": 30,
  "algoritmo": "score_baseado_em_frequencia_e_recencia"
}
```

---

## 🏆 **Card 02: Oportunidades na Rede**

### Endpoint
```
GET /services/api1/analytics/oportunidades-rede
```

### Parâmetros Query
```yaml
cnpj: "17.311.174/0001-78"          # OBRIGATÓRIO - CNPJ do franqueado
periodo_dias: 90                    # OPCIONAL - Período comparação (padrão: 90)
limite: 20                          # OPCIONAL - Top N oportunidades (padrão: 50)
```

### Headers
```
Authorization: Bearer {JWT_TOKEN}
Content-Type: application/json
```

### Exemplo de Request
```bash
curl -X GET "http://localhost:8089/services/api1/analytics/oportunidades-rede?cnpj=17.311.174/0001-78&periodo_dias=90&limite=20" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
```

### Response Estrutura
```json
{
  "success": true,
  "cnpj": "17.311.174/0001-78",
  "periodo_dias": 90,
  "oportunidades": [
    {
      "codigo_produto": "PA000025",
      "descricao_produto": "VITAMINA D3 2000UI",
      "categoria": "VITAMINAS",
      "seu_grupo": "A",
      "sua_quantidade": 20.0,
      "media_do_grupo": 45.0,
      "diferenca_percentual": -55.6,
      "unidades_adicionais": 25.0,
      "oportunidade_reais": 2400.00,
      "outros_franqueados_compram": 15,
      "nivel_prioridade": "ALTA",
      "score_prioridade": 85.2,
      "insight": "GRANDE OPORTUNIDADE: Você está 55% abaixo da média!",
      "recomendacao": "INCLUIR NO PRÓXIMO PEDIDO"
    }
  ],
  "total_oportunidades": 20,
  "algoritmo": "comparacao_vs_media_grupo_abc_corrigido",
  "versao": "card_02_oficial"
}
```

---

## 🔧 **Autenticação JWT**

### Login
```bash
curl -X POST "http://localhost:8089/services/api1/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "ArtesanalFC2025!"}'
```

### Response Login
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "expires_in": 86400,
  "token_type": "Bearer"
}
```

---

## 📈 **Algoritmos Implementados**

### Card 01: Score de Recompra
```sql
-- Fórmula: (Frequência * 10) / Dias desde última compra
score_recompra = COUNT(pedidos) * 10.0 / DATEDIFF(days, MAX(data_pedido), NOW())

-- Interpretação:
-- Score >= 3.0 = ALTA prioridade (compra frequente + recente)
-- Score >= 1.0 = MÉDIA prioridade (padrão moderado)
-- Score < 1.0  = BAIXA prioridade (compra esporádica)
```

### Card 02: Classificação ABC + Oportunidades
```sql
-- 1. Classificação ABC por volume (NTILE):
--    Grupo A = Top 33% franqueados por volume
--    Grupo B = Meio 33% franqueados
--    Grupo C = Últimos 33% franqueados

-- 2. Score de Priorização:
score = (diferenca_percentual * 0.5) + 
        (impacto_financeiro/100 * 0.3) + 
        (popularidade_rede * 0.2)
```

---

## 🚀 **Deploy & Testes**

### Testar Local
```bash
# 1. Compilar
cargo build --release

# 2. Rodar (desenvolvimento)
cargo run

# 3. Testar endpoints
.\test_endpoints.bat
```

### Testar Produção
```bash
# Health Check
curl https://conexao.artesanalfarmacia.com.br/services/api1/health

# Endpoints Cards (com token válido)
curl https://conexao.artesanalfarmacia.com.br/services/api1/analytics/recompra-inteligente?cnpj=17.311.174/0001-78
```

---

## 📋 **Status Implementação**

- ✅ **Card 01**: Query SQL oficial implementada
- ✅ **Card 02**: Query SQL com CTEs corrigidas implementada  
- ✅ **Estruturas Rust**: Todas as structs atualizadas
- ✅ **Endpoints**: Registrados e funcionais
- ✅ **Autenticação**: JWT obrigatório
- ✅ **Compilação**: Sucesso sem erros
- ⏳ **Testes**: Aguardando teste com dados reais
- ⏳ **Deploy**: Pronto para produção

---

**Queries baseadas nos documentos oficiais Card 01 e Card 02**  
**Implementação: 03/08/2025**  
**Status: ✅ Ready for Testing & Deploy**
