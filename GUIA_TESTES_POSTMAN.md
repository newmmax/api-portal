# 🧪 GUIA COMPLETO DE TESTES - POSTMAN

## 📋 **PASSO A PASSO PARA TESTAR OS CARDS ANALYTICS**

### **🚀 SETUP INICIAL (5 minutos)**

#### 1. **Importar Collection no Postman**
```
1. Abrir Postman
2. Clicar "Import" (botão superior esquerdo)
3. Selecionar arquivo: FC_Data_API_CARDS_ANALYTICS.postman_collection.json
4. Confirmar importação
```

#### 2. **Configurar Environment (Opcional)**
```yaml
# Se quiser usar environment ao invés de collection variables:
Environment Name: FC Data API - Local
Variables:
  base_url: http://localhost:8089/services/api1
  cnpj_teste: 17.311.174/0001-78
```

#### 3. **Verificar Collection Variables**
```
1. Clicar na Collection "FC Data API - Cards Analytics"
2. Aba "Variables"
3. Verificar se estão configuradas:
   - protocol: http
   - host: localhost  
   - port: 8089
   - api_path: /services/api1
   - cnpj_teste: 17.311.174/0001-78
```

---

### **🔥 EXECUÇÃO DOS TESTES (15 minutos)**

#### **ETAPA 1: Health Check (30 segundos)**
```
📍 Endpoint: 🏥 Health Check
🎯 Objetivo: Verificar se API está rodando

1. Selecionar request "🏥 Health Check"
2. Clicar "Send"
3. ✅ Verificar: Status 200 + response JSON com status

Expected Response:
{
  "status": "healthy",
  "databases": {
    "postgresql_fc": "connected",
    "sqlserver_portal": "connected",
    "sqlserver_protheus": "connected"
  }
}
```

#### **ETAPA 2: Autenticação (1 minuto)**
```
📍 Endpoint: 🔐 Autenticação > 🚪 Login
🎯 Objetivo: Obter token JWT

1. Expandir folder "🔐 Autenticação"
2. Selecionar "🚪 Login"
3. Verificar Body (já configurado):
   {
     "username": "admin",
     "password": "ArtesanalFC2025!"
   }
4. Clicar "Send"
5. ✅ Verificar: Status 200 + token capturado automaticamente
6. 🤖 Auto-magic: Token salvo nas variáveis da collection

Console Output Esperado:
🎉 TOKEN JWT CAPTURADO E SALVO!
🔑 Token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
⏰ Expira em: 86400 segundos
✅ Pronto para usar endpoints protegidos!
```

#### **ETAPA 3: Card 01 - Recompra Inteligente (2 minutos)**
```
📍 Endpoint: 🎯 CARDS ANALYTICS > 🔄 CARD 01: Recompra Inteligente
🎯 Objetivo: Testar algoritmo de IA para sugestões

1. Expandir folder "🎯 CARDS ANALYTICS"
2. Selecionar "🔄 CARD 01: Recompra Inteligente"
3. Verificar parâmetros (já configurados):
   - cnpj: {{cnpj_teste}} (17.311.174/0001-78)
   - periodo_dias: 180
   - limite: 30
4. Clicar "Send"
5. ✅ Verificar: Status 200 + dados de recompra

Expected Response Structure:
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
      "produtos_relacionados": [...]
    }
  ],
  "total_produtos": 25,
  "algoritmo": "score_baseado_em_frequencia_e_recencia"
}

Console Output Esperado:
🎯 CARD 01: Recompra Inteligente
📈 Total produtos: 25
🔍 Período: 180 dias
🏢 CNPJ: 17.311.174/0001-78
🔥 TOP 3 PRODUTOS:
1. PA000037 - Score: 4.2 (ALTA)
2. PA000045 - Score: 2.8 (MÉDIA)
3. PA000052 - Score: 1.9 (MÉDIA)
```

#### **ETAPA 4: Card 02 - Oportunidades na Rede (2 minutos)**
```
📍 Endpoint: 🎯 CARDS ANALYTICS > 🏆 CARD 02: Oportunidades na Rede
🎯 Objetivo: Testar análise comparativa vs rede

1. Selecionar "🏆 CARD 02: Oportunidades na Rede"
2. Verificar parâmetros (já configurados):
   - cnpj: {{cnpj_teste}} (17.311.174/0001-78)
   - periodo_dias: 90
   - limite: 20
3. Clicar "Send"
4. ✅ Verificar: Status 200 + oportunidades identificadas

Expected Response Structure:
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
  "total_oportunidades": 12,
  "algoritmo": "comparacao_vs_media_grupo_abc_corrigido",
  "versao": "card_02_oficial"
}

Console Output Esperado:
🏆 CARD 02: Oportunidades na Rede
📈 Total oportunidades: 12
🔍 Período: 90 dias
🏢 CNPJ: 17.311.174/0001-78
💰 TOP 3 OPORTUNIDADES:
1. PA000025 - R$ 2400 (ALTA)
2. PA000033 - R$ 800 (MÉDIA)
3. PA000041 - R$ 450 (BAIXA)
```

#### **ETAPA 5: Teste Automático Completo (1 minuto)**
```
📍 Endpoint: 📋 Exemplos Práticos > 🎯 Teste Completo Cards
🎯 Objetivo: Executar todos os Cards automaticamente

1. Expandir folder "📋 Exemplos Práticos"
2. Selecionar "🎯 Teste Completo Cards"
3. Clicar "Send"
4. 👀 Observar Console: Execução automática dos 2 Cards
5. ✅ Verificar: Ambos Cards executados com sucesso

Console Output Esperado:
🎯 EXECUTANDO TESTE COMPLETO DOS CARDS
================================
✅ CARD 01 - Recompra Inteligente: OK
📊 Produtos encontrados: 25
✅ CARD 02 - Oportunidades na Rede: OK
💰 Oportunidades encontradas: 12
================================
🎉 TESTE COMPLETO FINALIZADO!
```

---

### **🔧 TESTES ADICIONAIS (Opcional)**

#### **Validar Token JWT**
```
📍 Endpoint: 🔐 Autenticação > 🔍 Validar Token

1. Selecionar "🔍 Validar Token"
2. Clicar "Send"
3. ✅ Verificar: Token válido e informações do usuário
```

#### **Query Portal Customizada**
```
📍 Endpoint: 🔍 Portal Queries > 🎯 Query Portal Dinâmica

1. Selecionar "🎯 Query Portal Dinâmica"
2. Modificar Body se desejar (query SQL):
   {
     "query": "SELECT TOP 5 nome, cnpj FROM clientes WHERE deleted_at IS NULL"
   }
3. Clicar "Send"
4. ✅ Verificar: Dados do Portal retornados
```

#### **Dados FC (PostgreSQL)**
```
📍 Endpoint: 📊 Dados FC > 📈 Vendas FC

1. Selecionar "📈 Vendas FC"
2. Ajustar parâmetros se necessário
3. Clicar "Send"
4. ✅ Verificar: Dados históricos do FC
```

---

### **🚨 TROUBLESHOOTING**

#### **Problema: Token não capturado**
```yaml
Sintoma: Endpoints protegidos retornam 401
Solução:
  1. Executar Login novamente
  2. Verificar Console: deve mostrar "TOKEN JWT CAPTURADO"
  3. Verificar Variables da Collection: token deve estar preenchido
```

#### **Problema: Erro 500 nos Cards**
```yaml
Sintoma: Cards retornam erro interno
Possíveis causas:
  1. API não está rodando: Verificar Health Check
  2. Banco Portal desconectado: Verificar logs da API
  3. CNPJ não existe: Trocar cnpj_teste por um válido
  4. Sem dados no período: Aumentar periodo_dias
```

#### **Problema: API não responde**
```yaml
Sintoma: Timeout ou connection refused
Solução:
  1. Verificar se API está rodando: cargo run
  2. Verificar porta: deve ser 8089
  3. Verificar URL base: http://localhost:8089/services/api1
```

#### **Problema: Dados vazios nos Cards**
```yaml
Sintoma: total_produtos: 0 ou total_oportunidades: 0
Soluções:
  1. Trocar CNPJ para um com histórico de pedidos
  2. Aumentar periodo_dias (ex: 365 dias)
  3. Verificar se há dados no Portal
```

---

### **📊 INTERPRETAÇÃO DOS RESULTADOS**

#### **Card 01 - Scores de Recompra:**
```yaml
Score >= 3.0: 🔥 ALTA prioridade (compra frequente + recente)
Score >= 1.0: 🟡 MÉDIA prioridade (padrão moderado)
Score < 1.0:  🟢 BAIXA prioridade (compra esporádica)

Fórmula: (frequência_compra * 10) / dias_ultima_compra
```

#### **Card 02 - Oportunidades:**
```yaml
diferenca_percentual negativa = Oportunidade (está abaixo da média)
-50% ou mais = GRANDE OPORTUNIDADE
-30% a -49% = Oportunidade identificada
-29% ou menos = Pequena oportunidade

Grupos ABC:
- A: Top 33% franqueados por volume
- B: Meio 33% franqueados  
- C: Últimos 33% franqueados
```

---

### **🎯 CHECKLIST FINAL**

```yaml
✅ Collection importada no Postman
✅ Health Check: API respondendo
✅ Login: Token capturado automaticamente
✅ Card 01: Produtos de recompra retornados
✅ Card 02: Oportunidades identificadas
✅ Teste automático: Ambos Cards funcionando
✅ Console logs: Informações claras exibidas
✅ Responses: Estrutura JSON conforme documentado
```

---

**🚀 PRONTO PARA PRODUÇÃO!**

Todos os endpoints estão funcionais e testados. A API está preparada para deploy em produção com os Cards Analytics completamente implementados!

**📞 Suporte:** Se houver problemas, verificar logs da API com `cargo run` no terminal para diagnóstico detalhado.
