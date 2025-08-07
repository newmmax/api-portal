$ErrorActionPreference = "Continue"

# Navegar para o diretório do projeto
Set-Location "D:\PROJETOS\RUST\fc-data-api"

# Executar cargo check e capturar output
Write-Host "🚀 Testando compilação..." -ForegroundColor Green
$output = cargo check 2>&1

# Salvar resultado
$output | Out-File -FilePath "compile_test_result.txt" -Encoding UTF8

# Mostrar resultado
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ SUCESSO: Compilação sem erros!" -ForegroundColor Green
    Write-Host "Detalhes:" -ForegroundColor Cyan
    $output | Write-Host
} else {
    Write-Host "❌ ERRO: Problemas de compilação encontrados" -ForegroundColor Red
    Write-Host "Detalhes:" -ForegroundColor Yellow
    $output | Write-Host
}

Write-Host "`nResultado salvo em compile_test_result.txt" -ForegroundColor Cyan
