# ================================================
# IDENTIFICACAO CIRURGICA - CAMPO PROBLEMATICO
# ================================================

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "IDENTIFICAÇÃO CIRÚRGICA - CAMPO PROBLEMÁTICO" -ForegroundColor Cyan  
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

$TOKEN = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbl9wcm9kIiwiZXhwIjoxNzU0MTM3ODIyLCJpYXQiOjE3NTQwNTE0MjJ9.zbOytl3OPatV-0eQ1cvjTkVS1dIIoaEMqzccqUoWmSg"
$HEADERS = @{
    'Content-Type' = 'application/json'
    'Authorization' = "Bearer $TOKEN"
}
$BASE_URL = "https://conexao.artesanalfarmacia.com.br/services/api1/portal/query"

function Test-Field {
    param(
        [string]$FieldName,
        [string]$Description
    )
    
    Write-Host "🔍 TESTANDO: $FieldName ($Description)" -ForegroundColor Yellow
    
    try {
        $sql = "SELECT TOP 1 nome, $FieldName FROM clientes"
        $body = @{ query = $sql } | ConvertTo-Json
        $response = Invoke-RestMethod -Uri $BASE_URL -Method POST -Headers $HEADERS -Body $body
        
        if ($response.success) {
            Write-Host "✅ SUCESSO - Campo $FieldName OK" -ForegroundColor Green
            return $true
        } else {
            Write-Host "❌ FALHA - Campo $FieldName PROBLEMÁTICO" -ForegroundColor Red
            return $false
        }
    }
    catch {
        Write-Host "❌ ERRO - Campo $FieldName CAUSA PROBLEMA" -ForegroundColor Red
        Write-Host "Erro: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
}

Write-Host "BASELINE: SELECT nome FROM clientes (sabemos que funciona)" -ForegroundColor Green
Write-Host ""

# TESTE CADA CAMPO INDIVIDUALMENTE
$campos = @(
    @{nome="id"; desc="Primary Key (INT/BIGINT)"},
    @{nome="cod_totvs"; desc="Código Totvs (VARCHAR)"},
    @{nome="loja"; desc="Código Loja (VARCHAR)"},
    @{nome="email"; desc="Email (VARCHAR)"},
    @{nome="cnpj"; desc="CNPJ (VARCHAR com formatação)"},
    @{nome="cidade"; desc="Cidade (VARCHAR)"},
    @{nome="estado"; desc="Estado (VARCHAR)"},
    @{nome="created_at"; desc="Data Criação (DATETIME)"},
    @{nome="updated_at"; desc="Data Atualização (DATETIME)"},
    @{nome="deleted_at"; desc="Data Exclusão (DATETIME - pode ser NULL)"},
    @{nome="is_first_login"; desc="Primeiro Login (BIT/BOOLEAN)"},
    @{nome="grupo_venda"; desc="Grupo Venda (VARCHAR)"},
    @{nome="nome_fantasia"; desc="Nome Fantasia (VARCHAR)"}
)

$problematicos = @()
$funcionais = @()

foreach ($campo in $campos) {
    $sucesso = Test-Field -FieldName $campo.nome -Description $campo.desc
    
    if ($sucesso) {
        $funcionais += $campo.nome
    } else {
        $problematicos += $campo.nome
        Write-Host "🚨 CAMPO PROBLEMÁTICO IDENTIFICADO: $($campo.nome)" -ForegroundColor Red
        Write-Host "   Tipo: $($campo.desc)" -ForegroundColor Red
        Write-Host ""
    }
    
    Start-Sleep -Milliseconds 500  # Pequena pausa entre testes
}

Write-Host ""
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "RESULTADO DA IDENTIFICAÇÃO CIRÚRGICA" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

if ($problematicos.Count -gt 0) {
    Write-Host "❌ CAMPOS PROBLEMÁTICOS IDENTIFICADOS:" -ForegroundColor Red
    foreach ($campo in $problematicos) {
        Write-Host "   - $campo" -ForegroundColor Red
    }
} else {
    Write-Host "✅ TODOS OS CAMPOS INDIVIDUAIS FUNCIONAM" -ForegroundColor Green
    Write-Host "   Problema pode ser:" -ForegroundColor Yellow
    Write-Host "   - Combinação de campos específicos" -ForegroundColor Yellow
    Write-Host "   - Quantidade de dados (timeout)" -ForegroundColor Yellow
    Write-Host "   - Algum valor específico em um campo" -ForegroundColor Yellow
}

Write-Host ""
if ($funcionais.Count -gt 0) {
    Write-Host "✅ CAMPOS QUE FUNCIONAM:" -ForegroundColor Green
    foreach ($campo in $funcionais) {
        Write-Host "   - $campo" -ForegroundColor Green
    }
}

Write-Host ""
Write-Host "🎯 PRÓXIMO PASSO:" -ForegroundColor White
if ($problematicos.Count -gt 0) {
    Write-Host "   Corrigir conversão de tipos para os campos problemáticos identificados" -ForegroundColor Gray
} else {
    Write-Host "   Testar combinações de campos ou investigar timeout/valores específicos" -ForegroundColor Gray
}

Read-Host "Pressione Enter para continuar"
