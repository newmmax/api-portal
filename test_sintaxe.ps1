$ErrorActionPreference = "Continue"
Set-Location "D:\PROJETOS\RUST\fc-data-api"
Write-Host "🔧 Testando correcao do erro de sintaxe..." -ForegroundColor Cyan
$output = cargo check 2>&1
$output | Out-File -FilePath "sintaxe_test.log" -Encoding UTF8
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ SUCESSO: Erro de sintaxe corrigido!" -ForegroundColor Green
} else {
    Write-Host "❌ ERRO: Ainda há problemas" -ForegroundColor Red
}
$output | Write-Host
