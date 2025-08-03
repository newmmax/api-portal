# FC Data API - Pre-Validacao PowerShell
# Execute como Administrador!

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  FC DATA API - PRE-VALIDACAO DE DEPLOY" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""

# Verificar se é admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")
if (-not $isAdmin) {
    Write-Host "[ERRO] NAO ESTA EXECUTANDO COMO ADMINISTRADOR!" -ForegroundColor Red
    Write-Host "Execute o PowerShell como Administrador" -ForegroundColor Yellow
    Read-Host "Pressione Enter para sair"
    exit 1
}
Write-Host "[OK] Executando como Administrador" -ForegroundColor Green
Write-Host ""

# Mudar para o diretorio correto
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectPath = Split-Path -Parent $scriptPath
Set-Location $projectPath
Write-Host "Diretorio do projeto: $projectPath" -ForegroundColor Gray
Write-Host ""

# 1. Verificar executavel
Write-Host "[1/8] Verificando executavel..." -ForegroundColor Yellow
if (Test-Path "target\release\fc-data-api.exe") {
    Write-Host "[OK] Executavel encontrado" -ForegroundColor Green
    $exe = Get-Item "target\release\fc-data-api.exe"
    Write-Host "  Tamanho: $($exe.Length / 1MB) MB"
    Write-Host "  Modificado: $($exe.LastWriteTime)"
} else {
    Write-Host "[ERRO] Executavel nao encontrado!" -ForegroundColor Red
    Write-Host "Execute: cargo build --release" -ForegroundColor Yellow
    Read-Host "Pressione Enter para sair"
    exit 1
}
Write-Host ""

# 2. Verificar .env
Write-Host "[2/8] Verificando arquivo .env..." -ForegroundColor Yellow
if (Test-Path ".env") {
    Write-Host "[OK] Arquivo .env encontrado" -ForegroundColor Green
    Write-Host "Configuracoes principais:" -ForegroundColor Gray
    Get-Content ".env" | Where-Object { $_ -match "SERVER_PORT|API_PREFIX" -and $_ -notmatch "^#" } | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
} else {
    Write-Host "[ERRO] Arquivo .env nao encontrado!" -ForegroundColor Red
    Read-Host "Pressione Enter para sair"
    exit 1
}
Write-Host ""

# 3. Testar PostgreSQL
Write-Host "[3/8] Testando conexao PostgreSQL..." -ForegroundColor Yellow
try {
    $tcp = New-Object System.Net.Sockets.TcpClient
    $tcp.Connect("10.216.1.16", 5432)
    $tcp.Close()
    Write-Host "[OK] PostgreSQL acessivel em 10.216.1.16:5432" -ForegroundColor Green
} catch {
    Write-Host "[ERRO] PostgreSQL inacessivel!" -ForegroundColor Red
    Write-Host "Erro: $_" -ForegroundColor Gray
}
Write-Host ""

# 4. Verificar porta
Write-Host "[4/8] Verificando porta 8089..." -ForegroundColor Yellow
$port = Get-NetTCPConnection -LocalPort 8089 -ErrorAction SilentlyContinue
if ($port) {
    Write-Host "[AVISO] Porta 8089 em uso!" -ForegroundColor Red
    $port | Format-Table State, OwningProcess
    $response = Read-Host "Continuar mesmo assim? (S/N)"
    if ($response -ne "S") { exit 1 }
} else {
    Write-Host "[OK] Porta 8089 disponivel" -ForegroundColor Green
}
Write-Host ""

# 5. Verificar NSSM
Write-Host "[5/8] Verificando NSSM..." -ForegroundColor Yellow
$nssm = Get-Command nssm -ErrorAction SilentlyContinue
if ($nssm) {
    Write-Host "[OK] NSSM encontrado: $($nssm.Source)" -ForegroundColor Green
} elseif (Test-Path "C:\nssm\win64\nssm.exe") {
    Write-Host "[OK] NSSM encontrado em C:\nssm" -ForegroundColor Green
} else {
    Write-Host "[ERRO] NSSM nao encontrado!" -ForegroundColor Red
    Write-Host "Baixe de: https://nssm.cc" -ForegroundColor Yellow
}
Write-Host ""

# 6. Verificar servico existente
Write-Host "[6/8] Verificando servico existente..." -ForegroundColor Yellow
$service = Get-Service FCDataAPI -ErrorAction SilentlyContinue
if ($service) {
    Write-Host "[AVISO] Servico FCDataAPI ja existe!" -ForegroundColor Yellow
    Write-Host "  Status: $($service.Status)"
    Write-Host "  Sera necessario remove-lo primeiro"
} else {
    Write-Host "[OK] Nenhum servico conflitante" -ForegroundColor Green
}
Write-Host ""

# 7. Espaco em disco
Write-Host "[7/8] Verificando espaco em disco..." -ForegroundColor Yellow
$disk = Get-PSDrive C
$freeGB = [math]::Round($disk.Free / 1GB, 2)
if ($freeGB -gt 1) {
    Write-Host "[OK] Espaco livre: $freeGB GB" -ForegroundColor Green
} else {
    Write-Host "[AVISO] Pouco espaco: $freeGB GB" -ForegroundColor Yellow
}
Write-Host ""

# 8. Teste rapido da API
Write-Host "[8/8] Teste rapido da API..." -ForegroundColor Yellow
Write-Host "Iniciando API para teste (15 segundos)..." -ForegroundColor Gray
$process = Start-Process "target\release\fc-data-api.exe" -PassThru -WindowStyle Hidden
Start-Sleep -Seconds 10

try {
    $response = Invoke-WebRequest -Uri "http://localhost:8089/services/api1/health" -UseBasicParsing
    if ($response.StatusCode -eq 200) {
        Write-Host "[OK] API respondendo corretamente!" -ForegroundColor Green
        $response.Content
    }
} catch {
    Write-Host "[ERRO] API nao respondeu!" -ForegroundColor Red
}

Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
Write-Host ""

# Resumo
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  VALIDACAO CONCLUIDA" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Proximo passo: Execute 02_BACKUP_ATUAL.bat" -ForegroundColor Yellow
Write-Host ""
Read-Host "Pressione Enter para sair"
