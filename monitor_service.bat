@echo off
REM ===========================================
REM FC Data API - Monitor de Serviço
REM ===========================================

:INICIO
cls
echo =========================================
echo    FC Data API - Monitor de Servico
echo =========================================
echo Data/Hora: %date% %time%
echo =========================================
echo.

set SERVICE_NAME=FCDataAPI
set INSTALL_DIR=C:\fcdata-api
set API_URL=http://localhost:8080/services/api1

REM Verificar status do serviço
echo [1] STATUS DO SERVICO
echo ---------------------
sc query %SERVICE_NAME% 2>nul | findstr /C:"SERVICE_NAME" /C:"STATE" /C:"WIN32_EXIT_CODE"

REM Verificar se está rodando
sc query %SERVICE_NAME% | findstr "RUNNING" >nul
if %errorLevel% EQU 0 (
    echo.
    echo [+] Servico esta RODANDO
) else (
    echo.
    echo [!] ATENCAO: Servico NAO esta rodando!
    echo.
    choice /C SN /M "Deseja iniciar o servico? (S/N)"
    if errorlevel 2 goto CONTINUAR
    if errorlevel 1 (
        echo Iniciando servico...
        nssm start %SERVICE_NAME%
        timeout /t 3 /nobreak >nul
    )
)
:CONTINUAR

echo.
echo [2] PROCESSOS E PORTAS
echo ----------------------
REM Verificar processo
tasklist | findstr /I "fc-data-api.exe" 2>nul
if %errorLevel% NEQ 0 (
    echo [!] Processo nao encontrado
) else (
    echo [+] Processo encontrado
)

REM Verificar porta
netstat -ano | findstr :8080 | findstr LISTENING >nul
if %errorLevel% EQU 0 (
    echo [+] Porta 8080 esta escutando
) else (
    echo [!] Porta 8080 NAO esta escutando
)

echo.
echo [3] HEALTH CHECK DA API
echo -----------------------
powershell -Command "try { $r = Invoke-WebRequest -Uri '%API_URL%/health' -TimeoutSec 5 -UseBasicParsing; Write-Host '[+] API respondendo - Status:' $r.StatusCode } catch { Write-Host '[!] API nao responde:' $_.Exception.Message }"

echo.
echo [4] ULTIMAS LINHAS DOS LOGS
echo ---------------------------
if exist "%INSTALL_DIR%\logs\service.log" (
    echo === SERVICE.LOG ===
    powershell -Command "Get-Content '%INSTALL_DIR%\logs\service.log' -Tail 5"
) else (
    echo [!] Arquivo de log nao encontrado
)

echo.
if exist "%INSTALL_DIR%\logs\error.log" (
    echo === ERROR.LOG ===
    powershell -Command "Get-Content '%INSTALL_DIR%\logs\error.log' -Tail 5"
    REM Verificar se há erros recentes
    powershell -Command "$content = Get-Content '%INSTALL_DIR%\logs\error.log' -Tail 20; if ($content) { Write-Host '[!] ERROS ENCONTRADOS NO LOG!' -ForegroundColor Red }"
) else (
    echo [!] Arquivo de erro nao encontrado
)

echo.
echo [5] ESTATISTICAS
echo ----------------
REM Tamanho dos logs
for %%A in ("%INSTALL_DIR%\logs\service.log") do echo Tamanho service.log: %%~zA bytes
for %%A in ("%INSTALL_DIR%\logs\error.log") do echo Tamanho error.log: %%~zA bytes

REM Uptime aproximado (última modificação do PID file ou log)
echo.
powershell -Command "$log = Get-Item '%INSTALL_DIR%\logs\service.log' -ErrorAction SilentlyContinue; if ($log) { $uptime = (Get-Date) - $log.LastWriteTime; Write-Host 'Uptime aproximado:' $uptime.ToString() }"

echo.
echo =========================================
echo [A] Atualizar  [L] Ver logs completos
echo [R] Reiniciar  [P] Parar servico
echo [I] Iniciar    [Q] Sair
echo =========================================
choice /C ALRPIQ /N /M "Escolha uma opcao: "

if errorlevel 6 goto FIM
if errorlevel 5 goto INICIAR
if errorlevel 4 goto PARAR
if errorlevel 3 goto REINICIAR
if errorlevel 2 goto LOGS
if errorlevel 1 goto INICIO

:INICIAR
echo.
echo Iniciando servico...
nssm start %SERVICE_NAME%
timeout /t 3 /nobreak >nul
goto INICIO

:PARAR
echo.
echo Parando servico...
nssm stop %SERVICE_NAME%
timeout /t 3 /nobreak >nul
goto INICIO

:REINICIAR
echo.
echo Reiniciando servico...
nssm restart %SERVICE_NAME%
timeout /t 5 /nobreak >nul
goto INICIO

:LOGS
echo.
echo Abrindo logs...
if exist "%INSTALL_DIR%\logs\service.log" notepad "%INSTALL_DIR%\logs\service.log"
if exist "%INSTALL_DIR%\logs\error.log" start notepad "%INSTALL_DIR%\logs\error.log"
goto INICIO

:FIM
echo.
echo Monitor encerrado.
