# ================================================
# INVESTIGACAO PROFISSIONAL - PORTAL QUERY DEBUG
# ================================================

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "INVESTIGACAO PROFISSIONAL - PORTAL QUERY DEBUG" -ForegroundColor Cyan  
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

$TOKEN = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbl9wcm9kIiwiZXhwIjoxNzU0MTM3ODIyLCJpYXQiOjE3NTQwNTE0MjJ9.zbOytl3OPatV-0eQ1cvjTkVS1dIIoaEMqzccqUoWmSg"
$HEADERS = @{
    'Content-Type' = 'application/json'
    'Authorization' = "Bearer $TOKEN"
}
$BASE_URL = "http://localhost:8089/services/api1/portal/query"

function Test-Query {
    param(
        [string]$TestName,
        [string]$SQL,
        [string]$Description
    )
    
    Write-Host "TESTE $TestName`: $Description" -ForegroundColor Yellow
    Write-Host "SQL: $SQL" -ForegroundColor Gray
    
    try {
        $body = @{ sql = $SQL } | ConvertTo-Json
        $response = Invoke-RestMethod -Uri $BASE_URL -Method POST -Headers $HEADERS -Body $body
        
        if ($response.success) {
            Write-Host "✅ SUCESSO - Retornou $($response.count) registros" -ForegroundColor Green
            if ($response.count -gt 0 -and $response.data.Count -gt 0) {
                Write-Host "Exemplo de dados:" -ForegroundColor Gray
                $response.data[0] | ConvertTo-Json -Compress | Write-Host
            }
        } else {
            Write-Host "❌ FALHA - API retornou erro" -ForegroundColor Red
            Write-Host $response | ConvertTo-Json -Depth 3
        }
    }
    catch {
        Write-Host "❌ ERRO HTTP - Falha na requisição" -ForegroundColor Red
        Write-Host "Erro: $($_.Exception.Message)" -ForegroundColor Red
        if ($_.Exception.Response) {
            Write-Host "Status: $($_.Exception.Response.StatusCode)" -ForegroundColor Red
        }
    }
    
    Write-Host ""
    Write-Host "---" -ForegroundColor DarkGray
    Write-Host ""
}

# TESTE 1: Baseline (já confirmado funcionando)
Test-Query "1" "SELECT 1 as test" "Query baseline simples"

# TESTE 2: Verificar tabela
Test-Query "2" "SELECT COUNT(*) as total FROM clientes" "Verificar se tabela clientes existe"

# TESTE 3: Campos básicos
Test-Query "3" "SELECT TOP 1 id, nome FROM clientes" "Query simples com campos básicos"

# TESTE 4: Adicionar mais campos
Test-Query "4" "SELECT TOP 1 id, nome, email, cnpj FROM clientes" "Adicionar campos texto"

# TESTE 5: Testar campos de data convertidos
Test-Query "5" "SELECT TOP 1 id, CONVERT(varchar, created_at, 120) as created_at FROM clientes" "Testar conversão de data"

# TESTE 6: Testar campo booleano
Test-Query "6" "SELECT TOP 1 id, CAST(is_first_login as int) as is_first_login FROM clientes" "Testar conversão booleano"

# TESTE 7: Todos os campos básicos sem conversões
Test-Query "7" "SELECT TOP 1 id, cod_totvs, loja, nome, email, cnpj, cidade, estado FROM clientes" "Campos básicos sem conversões"

# TESTE 8: Adicionar campos de data originais (sem conversão)
Test-Query "8" "SELECT TOP 1 id, nome, created_at, updated_at, deleted_at FROM clientes" "Campos de data originais"

# TESTE 9: Query original COMPLETA (a que deve falhar)
$QUERY_ORIGINAL = "SELECT id, cod_totvs, loja, nome, email, cnpj, cidade, estado, CONVERT(varchar, created_at, 120) as created_at, CONVERT(varchar, updated_at, 120) as updated_at, CONVERT(varchar, deleted_at, 120) as deleted_at, CAST(is_first_login as int) as is_first_login, grupo_venda, nome_fantasia FROM clientes"
Test-Query "9" $QUERY_ORIGINAL "Query original COMPLETA (problemática)"

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "ANÁLISE COMPLETA" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "INSTRUÇÕES PARA ANÁLISE:" -ForegroundColor White
Write-Host "1. Observe qual teste falha primeiro" -ForegroundColor Gray
Write-Host "2. O último teste que funciona indica onde está o problema" -ForegroundColor Gray
Write-Host "3. Compare campos entre teste que funciona vs teste que falha" -ForegroundColor Gray
Write-Host "4. Identifique tipo SQL Server problemático" -ForegroundColor Gray
Write-Host "5. Se todos os testes funcionarem, problema pode ser de timeout ou quantidade de dados" -ForegroundColor Gray
Write-Host ""

Read-Host "Pressione Enter para continuar"
