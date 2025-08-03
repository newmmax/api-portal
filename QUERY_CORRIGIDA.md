# 📋 FC Data API - Query Única Implementada

## ✅ Correção Aplicada

Você estava certo! A API agora usa **exatamente a query que você forneceu** e que funciona no DBeaver.

## 🔍 O que foi ajustado:

1. **Query única**: Removida a separação entre "resumo" e "detalhada" - agora usa apenas sua query completa
2. **Tipos de dados**: Ajustados os tipos para corresponder ao banco PostgreSQL
3. **Filtros de data**: Convertidos de String para NaiveDate automaticamente

## 📡 Endpoints disponíveis:

### 1. Vendas (Query completa)
```
GET /services/api1/data/vendas
GET /services/api1/data/vendas/detalhes  (mesma query)
```

**Parâmetros opcionais:**
- `data_inicio` - formato: YYYY-MM-DD
- `data_fim` - formato: YYYY-MM-DD  
- `empresa` - nome da empresa
- `filial` - código da filial
- `vendedor` - código do vendedor
- `produto` - nome do produto (busca parcial)
- `limite` - número máximo de registros

### 2. Debug da Query (sem autenticação)
```
GET /services/api1/debug/query
```
Mostra exatamente qual query SQL será executada com os filtros aplicados.

## 🧪 Como testar:

1. **Teste rápido da query:**
```bash
test_query.bat
```

2. **Ver a query que será executada:**
```
http://localhost:8080/services/api1/debug/query?data_inicio=2024-01-01&limite=10
```

3. **Executar a query real (com autenticação):**
```bash
# Login primeiro
curl -X POST http://localhost:8080/services/api1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"ArtesanalFC2025!"}'

# Usar o token retornado
curl -X GET "http://localhost:8080/services/api1/data/vendas?limite=10" \
  -H "Authorization: Bearer {TOKEN}"
```

## 🔧 Query implementada:

```sql
SELECT 
    cab.companygroupname,
    cfg.cnpj,
    cab.cdfil,
    emp.descrfil,
    cab.nrcpm,
    cab.dtpagefe,
    cab.dteminfce,
    cab.cdcli,
    cli.nomecli,
    cab.cdfunre,
    ven.nomefun,
    it.itemid,
    it.cdpro,
    pr.descrprd,
    pr.setor,
    it.quant,
    it.pruni,
    it.vrtot,
    it.vrdsc,
    it.vrrcb,
    it.prcusto,
    it.prcompra
FROM FC14000 as cab
INNER JOIN (SELECT company_id, cnpj, companygroupname FROM company_config) cfg
    ON cab.company_id = cfg.company_id AND cab.companygroupname = cfg.companygroupname
LEFT JOIN (SELECT company_id, cdcli, nomecli FROM fc07000) cli
    ON cab.company_id = cli.company_id AND cab.cdcli = cli.cdcli
INNER JOIN (SELECT company_id, cdfun, nomefun FROM fc08000 GROUP BY company_id, cdfun, nomefun) ven
    ON cab.company_id = ven.company_id AND cab.cdfunre = ven.cdfun
INNER JOIN (
    SELECT company_id, cdfil, nrcpm, itemid, cdpro, quant,
    CAST(pruni as numeric) pruni, 
    CAST(vrtot as numeric) vrtot,
    CAST(vrdsc as numeric) vrdsc,
    ROUND(CAST((vrtot+vrtxav) - (vrdsc + vrdscv) as numeric),2) vrrcb,
    prcusto, prcompra 
    FROM fc14100
) it
    ON it.company_id = cab.company_id AND it.cdfil = cab.cdfil AND it.nrcpm = cab.nrcpm
LEFT JOIN (SELECT company_id, cdpro, descrprd, setor FROM fc03000 pr WHERE 1=1) pr
    ON it.company_id = pr.company_id AND it.cdpro = pr.cdpro
INNER JOIN (SELECT company_id, cdfil, descrfil FROM companies) emp
    ON cab.company_id = emp.company_id AND cab.cdfil = emp.cdfil
INNER JOIN (SELECT company_id, company_name FROM company_config) cc
    ON cab.company_id = cc.company_id
WHERE pr.cdpro IS NOT NULL
ORDER BY cab.dtpagefe, cab.company_id
```

## 🎯 Resposta JSON:

```json
{
  "success": true,
  "data": [
    {
      "companygroupname": "GRUPO01",
      "cnpj": "12345678901234",
      "cdfil": 1,
      "descrfil": "FILIAL 01",
      "nrcpm": 12345,
      "dtpagefe": "2024-01-15",
      "dteminfce": "2024-01-15",
      "cdcli": 100,
      "nomecli": "CLIENTE TESTE",
      "cdfunre": 10,
      "nomefun": "VENDEDOR 01",
      "itemid": 1,
      "cdpro": 1001,
      "descrprd": "PRODUTO TESTE",
      "setor": "01",
      "quant": 2.0,
      "pruni": 10.50,
      "vrtot": 21.00,
      "vrdsc": 1.00,
      "vrrcb": 20.00,
      "prcusto": 8.00,
      "prcompra": 7.50
    }
  ],
  "total": 1
}
```

## ❗ Se ainda houver erro:

1. Verifique se a API está rodando: `test_api.bat`
2. Use o endpoint de debug para ver a query exata: `/debug/query`
3. Compare com a query que funciona no DBeaver
4. Verifique os logs em: `C:\fcdata-api\logs\`

A query agora é **exatamente** a que você forneceu!
