$ErrorActionPreference = "Continue"
Set-Location "D:\PROJETOS\RUST\fc-data-api"

Write-Host "🔧 Testando correção final dos borrows..." -ForegroundColor Cyan

# Executar cargo check
Write-Host "Executando cargo check..." -ForegroundColor Yellow
$output = cargo check 2>&1

# Salvar resultado
$output | Out-File -FilePath "borrow_check_resultado.txt" -Encoding UTF8

# Verificar resultado
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ SUCESSO: Borrows corrigidos! Compilação limpa." -ForegroundColor Green
    Write-Host "🚀 Sistema pronto para build final!" -ForegroundColor Cyan
} else {
    Write-Host "❌ ERRO: Ainda há problemas de compilação" -ForegroundColor Red
    Write-Host "Detalhes:" -ForegroundColor Yellow
}

# Mostrar output
$output | Write-Host

Write-Host "`nResultado completo salvo em borrow_check_resultado.txt" -ForegroundColor Blue
