# 📊 Queries SQL de Exemplo - FC Data API

## Queries Úteis para Análise de Dados

### 1. Dashboard Geral

```sql
-- Resumo geral de vendas
SELECT 
    COUNT(DISTINCT nrcpm) as total_cupons,
    COUNT(DISTINCT cdcli) as total_clientes,
    COUNT(DISTINCT cdfil) as total_filiais,
    SUM(vrrcb) as faturamento_total,
    AVG(vrrcb) as ticket_medio
FROM fc14100
WHERE dtsai >= CURRENT_DATE - INTERVAL '30 days'
```

### 2. Análise por Período

```sql
-- Vendas por mês
SELECT 
    EXTRACT(YEAR FROM dtsai::date) as ano,
    EXTRACT(MONTH FROM dtsai::date) as mes,
    COUNT(DISTINCT nrcpm) as num_vendas,
    SUM(vrrcb) as faturamento
FROM fc14100
WHERE dtsai IS NOT NULL
GROUP BY ano, mes
ORDER BY ano DESC, mes DESC
LIMIT 12
```

```sql
-- Comparativo ano a ano
SELECT 
    EXTRACT(YEAR FROM dtsai::date) as ano,
    COUNT(DISTINCT nrcpm) as vendas,
    SUM(vrrcb) as faturamento,
    AVG(vrrcb) as ticket_medio
FROM fc14100
WHERE EXTRACT(YEAR FROM dtsai::date) IN (2024, 2025)
GROUP BY ano
```

### 3. Análise de Produtos

```sql
-- Top 20 produtos por faturamento
SELECT 
    p.cdpro,
    p.descrprd as produto,
    p.setor,
    SUM(v.quant) as qtd_vendida,
    SUM(v.vrrcb) as faturamento,
    AVG(v.pruni) as preco_medio
FROM fc14100 v
INNER JOIN fc03000 p ON v.cdpro = p.cdpro AND v.company_id = p.company_id
WHERE v.dtsai >= CURRENT_DATE - INTERVAL '90 days'
GROUP BY p.cdpro, p.descrprd, p.setor
ORDER BY faturamento DESC
LIMIT 20
```

```sql
-- Produtos com maior margem
SELECT 
    p.cdpro,
    p.descrprd as produto,
    AVG(v.pruni) as preco_venda,
    AVG(v.prcusto) as preco_custo,
    AVG((v.pruni - v.prcusto) / NULLIF(v.pruni, 0) * 100) as margem_percent
FROM fc14100 v
INNER JOIN fc03000 p ON v.cdpro = p.cdpro
WHERE v.prcusto > 0 AND v.pruni > 0
GROUP BY p.cdpro, p.descrprd
HAVING COUNT(*) > 10
ORDER BY margem_percent DESC
LIMIT 20
```

### 4. Análise de Clientes

```sql
-- Top clientes por faturamento
SELECT 
    c.cdcli,
    c.nomecli as cliente,
    COUNT(DISTINCT v.nrcpm) as num_compras,
    SUM(i.vrrcb) as faturamento_total,
    AVG(i.vrrcb) as ticket_medio,
    MAX(v.dtpagefe) as ultima_compra
FROM fc14000 v
INNER JOIN fc14100 i ON v.nrcpm = i.nrcpm AND v.cdfil = i.cdfil
LEFT JOIN fc07000 c ON v.cdcli = c.cdcli AND v.company_id = c.company_id
WHERE v.dtpagefe >= CURRENT_DATE - INTERVAL '180 days'
GROUP BY c.cdcli, c.nomecli
ORDER BY faturamento_total DESC
LIMIT 50
```

```sql
-- Clientes novos vs recorrentes
WITH primeira_compra AS (
    SELECT 
        cdcli,
        MIN(dtpagefe) as data_primeira_compra
    FROM fc14000
    GROUP BY cdcli
)
SELECT 
    CASE 
        WHEN pc.data_primeira_compra >= CURRENT_DATE - INTERVAL '30 days' 
        THEN 'Novo'
        ELSE 'Recorrente'
    END as tipo_cliente,
    COUNT(DISTINCT v.cdcli) as qtd_clientes,
    COUNT(DISTINCT v.nrcpm) as num_vendas,
    SUM(i.vrrcb) as faturamento
FROM fc14000 v
INNER JOIN fc14100 i ON v.nrcpm = i.nrcpm
INNER JOIN primeira_compra pc ON v.cdcli = pc.cdcli
WHERE v.dtpagefe >= CURRENT_DATE - INTERVAL '30 days'
GROUP BY tipo_cliente
```

### 5. Análise de Vendedores

```sql
-- Performance de vendedores
SELECT 
    f.cdfun,
    f.nomefun as vendedor,
    COUNT(DISTINCT v.nrcpm) as num_vendas,
    SUM(i.vrrcb) as faturamento,
    AVG(i.vrrcb) as ticket_medio,
    COUNT(DISTINCT v.cdcli) as clientes_atendidos
FROM fc14000 v
INNER JOIN fc14100 i ON v.nrcpm = i.nrcpm
INNER JOIN fc08000 f ON v.cdfunre = f.cdfun AND v.company_id = f.company_id
WHERE v.dtpagefe >= CURRENT_DATE - INTERVAL '30 days'
GROUP BY f.cdfun, f.nomefun
ORDER BY faturamento DESC
```

### 6. Análise por Filial

```sql
-- Comparativo entre filiais
SELECT 
    e.cdfil,
    e.descrfil as filial,
    e.munic as cidade,
    COUNT(DISTINCT v.nrcpm) as num_vendas,
    SUM(i.vrrcb) as faturamento,
    COUNT(DISTINCT v.cdcli) as clientes_unicos
FROM fc14000 v
INNER JOIN fc14100 i ON v.nrcpm = i.nrcpm AND v.cdfil = i.cdfil
INNER JOIN companies e ON v.cdfil = e.cdfil AND v.company_id = e.company_id
WHERE v.dtpagefe >= CURRENT_DATE - INTERVAL '30 days'
GROUP BY e.cdfil, e.descrfil, e.munic
ORDER BY faturamento DESC
```

### 7. Análise de Descontos

```sql
-- Impacto dos descontos nas vendas
SELECT 
    DATE_TRUNC('month', dtsai::date) as mes,
    SUM(vrtot) as valor_bruto,
    SUM(vrdsc + vrdscv) as total_descontos,
    SUM(vrrcb) as valor_liquido,
    AVG((vrdsc + vrdscv) / NULLIF(vrtot, 0) * 100) as percent_desconto_medio
FROM fc14100
WHERE dtsai >= CURRENT_DATE - INTERVAL '6 months'
GROUP BY mes
ORDER BY mes DESC
```

### 8. Análise de Setores

```sql
-- Performance por setor de produtos
SELECT 
    p.setor,
    COUNT(DISTINCT p.cdpro) as num_produtos,
    SUM(v.quant) as qtd_vendida,
    SUM(v.vrrcb) as faturamento,
    AVG(v.ptlucro) as margem_media
FROM fc14100 v
INNER JOIN fc03000 p ON v.cdpro = p.cdpro
WHERE v.dtsai >= CURRENT_DATE - INTERVAL '30 days'
  AND p.setor IS NOT NULL
GROUP BY p.setor
ORDER BY faturamento DESC
```

### 9. Análise de Horários (se tiver campo de hora)

```sql
-- Vendas por dia da semana
SELECT 
    EXTRACT(DOW FROM dtpagefe) as dia_semana,
    CASE EXTRACT(DOW FROM dtpagefe)
        WHEN 0 THEN 'Domingo'
        WHEN 1 THEN 'Segunda'
        WHEN 2 THEN 'Terça'
        WHEN 3 THEN 'Quarta'
        WHEN 4 THEN 'Quinta'
        WHEN 5 THEN 'Sexta'
        WHEN 6 THEN 'Sábado'
    END as nome_dia,
    COUNT(DISTINCT nrcpm) as num_vendas,
    SUM(vrrcb) as faturamento
FROM fc14100
WHERE dtpagefe >= CURRENT_DATE - INTERVAL '30 days'
GROUP BY dia_semana
ORDER BY dia_semana
```

### 10. Análise ABC de Produtos

```sql
-- Curva ABC de produtos
WITH produto_faturamento AS (
    SELECT 
        p.cdpro,
        p.descrprd,
        SUM(v.vrrcb) as faturamento,
        SUM(SUM(v.vrrcb)) OVER () as faturamento_total
    FROM fc14100 v
    INNER JOIN fc03000 p ON v.cdpro = p.cdpro
    WHERE v.dtsai >= CURRENT_DATE - INTERVAL '90 days'
    GROUP BY p.cdpro, p.descrprd
),
produto_ranking AS (
    SELECT 
        *,
        faturamento / faturamento_total * 100 as percent_faturamento,
        SUM(faturamento / faturamento_total * 100) OVER (ORDER BY faturamento DESC) as percent_acumulado
    FROM produto_faturamento
)
SELECT 
    cdpro,
    descrprd,
    faturamento,
    percent_faturamento,
    percent_acumulado,
    CASE 
        WHEN percent_acumulado <= 80 THEN 'A'
        WHEN percent_acumulado <= 95 THEN 'B'
        ELSE 'C'
    END as classe_abc
FROM produto_ranking
ORDER BY faturamento DESC
LIMIT 100
```

## 📝 Notas Importantes

1. **Performance**: Sempre use filtros de data para melhorar a performance
2. **Índices**: Certifique-se de que existem índices em:
   - `dtsai`, `dtpagefe` (datas)
   - `cdpro`, `cdcli`, `cdfun` (chaves)
   - `company_id`, `cdfil` (particionamento)
3. **Limites**: Use sempre LIMIT em queries exploratórias
4. **NULL Values**: Trate valores NULL apropriadamente
5. **Type Casting**: Use `::date` ou `::timestamp` quando necessário

## 🔍 Dicas de Otimização

- Para grandes volumes, considere criar views materializadas
- Use EXPLAIN ANALYZE para identificar gargalos
- Particione tabelas por data se o volume for muito alto
- Considere índices compostos para queries frequentes

## 💡 Queries para Monitoramento

```sql
-- Verificar volume de dados
SELECT 
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) as size
FROM pg_tables
WHERE tablename IN ('fc14000', 'fc14100', 'fc03000', 'fc07000', 'fc08000')
ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;

-- Verificar índices
SELECT 
    schemaname,
    tablename,
    indexname,
    indexdef
FROM pg_indexes
WHERE tablename IN ('fc14000', 'fc14100', 'fc03000', 'fc07000', 'fc08000');
```
