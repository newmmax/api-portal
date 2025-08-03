@echo off
REM ===========================================
REM FC Data API - Painel de Controle
REM ===========================================
REM Central de gerenciamento do servico

:MENU
cls
echo.
echo    _/_/_/_/    _/_/_/        _/_/_/      _/_/    _/_/_/_/_/    _/_/   
echo   _/        _/              _/    _/  _/    _/      _/      _/    _/  
echo  _/_/_/    _/              _/    _/  _/_/_/_/      _/      _/_/_/_/   
echo _/        _/              _/    _/  _/    _/      _/      _/    _/    
echo _/          _/_/_/        _/_/_/    _/    _/      _/      _/    _/     
echo.
echo               PAINEL DE CONTROLE - FC DATA API
echo =======================================================
echo.
echo   [1] Status do Servico      [6] Configurar Tarefas
echo   [2] Iniciar Servico        [7] Backup Manual
echo   [3] Parar Servico          [8] Ver Logs
echo   [4] Reiniciar Servico      [9] Testar API
echo   [5] Monitor em Tempo Real  [0] Documentacao
echo.
echo   [I] Instalar Completo      [U] Desinstalar
echo   [Q] Sair
echo.
echo =======================================================
choice /C 1234567890IUQ /N /M "Selecione uma opcao: "

if errorlevel 13 goto SAIR
if errorlevel 12 goto DESINSTALAR
if errorlevel 11 goto INSTALAR
if errorlevel 10 goto DOCS
if errorlevel 9 goto TESTAR_API
if errorlevel 8 goto VER_LOGS
if errorlevel 7 goto BACKUP_MANUAL
if errorlevel 6 goto CONFIG_TAREFAS
if errorlevel 5 goto MONITOR
if errorlevel 4 goto REINICIAR
if errorlevel 3 goto PARAR
if errorlevel 2 goto INICIAR
if errorlevel 1 goto STATUS

:STATUS
cls
echo =======================================================
echo STATUS DO SERVICO
echo =======================================================
echo.
sc query FCDataAPI | findstr /C:"SERVICE_NAME" /C:"STATE" /C:"WIN32_EXIT_CODE"
echo.
netstat -ano | findstr :8080 | findstr LISTENING >nul
if %errorLevel% EQU 0 (
    echo [+] Porta 8080 esta ATIVA
) else (
    echo [!] Porta 8080 NAO esta escutando
)
echo.
powershell -Command "try { $r = Invoke-WebRequest -Uri 'http://localhost:8080/services/api1/health' -TimeoutSec 3 -UseBasicParsing; Write-Host '[+] API respondendo - Status:' $r.StatusCode } catch { Write-Host '[!] API nao responde' }"
echo.
pause
goto MENU

:INICIAR
cls
echo Iniciando servico FCDataAPI...
nssm start FCDataAPI
timeout /t 3 /nobreak >nul
echo.
sc query FCDataAPI | findstr "STATE"
echo.
pause
goto MENU

:PARAR
cls
echo Parando servico FCDataAPI...
nssm stop FCDataAPI
timeout /t 3 /nobreak >nul
echo.
sc query FCDataAPI | findstr "STATE"
echo.
pause
goto MENU

:REINICIAR
cls
echo Reiniciando servico FCDataAPI...
nssm restart FCDataAPI
timeout /t 5 /nobreak >nul
echo.
sc query FCDataAPI | findstr "STATE"
echo.
pause
goto MENU

:MONITOR
cls
echo Abrindo monitor em tempo real...
start cmd /k "cd /d %~dp0 && monitor_service.bat"
goto MENU

:CONFIG_TAREFAS
cls
echo Configurando tarefas agendadas...
echo.
call setup_scheduled_tasks.bat
goto MENU

:BACKUP_MANUAL
cls
echo Executando backup manual...
echo.
call backup_system.bat
goto MENU

:VER_LOGS
cls
echo =======================================================
echo LOGS DO SISTEMA
echo =======================================================
echo.
echo [1] Service.log (Notepad)
echo [2] Error.log (Notepad)
echo [3] Ultimas 20 linhas (Console)
echo [4] Pasta de logs (Explorer)
echo [0] Voltar
echo.
choice /C 12340 /N /M "Selecione: "

if errorlevel 5 goto MENU
if errorlevel 4 start explorer "C:\fcdata-api\logs" && goto VER_LOGS
if errorlevel 3 (
    cls
    echo === ULTIMAS 20 LINHAS - SERVICE.LOG ===
    powershell -Command "Get-Content 'C:\fcdata-api\logs\service.log' -Tail 20"
    echo.
    echo === ULTIMAS 20 LINHAS - ERROR.LOG ===
    powershell -Command "Get-Content 'C:\fcdata-api\logs\error.log' -Tail 20"
    echo.
    pause
    goto VER_LOGS
)
if errorlevel 2 start notepad "C:\fcdata-api\logs\error.log" && goto VER_LOGS
if errorlevel 1 start notepad "C:\fcdata-api\logs\service.log" && goto VER_LOGS

:TESTAR_API
cls
echo Testando endpoints da API...
echo.
call test_endpoints.bat
goto MENU

:DOCS
cls
echo =======================================================
echo DOCUMENTACAO
echo =======================================================
echo.
echo [1] README Principal
echo [2] Guia de Deployment Windows
echo [3] Guia do Postman
echo [4] Exemplos de Queries SQL
echo [5] Abrir pasta do projeto
echo [0] Voltar
echo.
choice /C 123450 /N /M "Selecione: "

if errorlevel 6 goto MENU
if errorlevel 5 start explorer "%~dp0" && goto DOCS
if errorlevel 4 start notepad "QUERIES_EXEMPLOS.sql" && goto DOCS
if errorlevel 3 start notepad "POSTMAN_README.md" && goto DOCS
if errorlevel 2 start notepad "DEPLOYMENT_WINDOWS.md" && goto DOCS
if errorlevel 1 start notepad "README.md" && goto DOCS

:INSTALAR
cls
echo.
echo ATENCAO: Isto ira instalar o servico completo.
echo Certifique-se de estar executando como Administrador!
echo.
pause
call install_complete.bat
goto MENU

:DESINSTALAR
cls
echo.
echo ATENCAO: Isto ira REMOVER o servico do Windows!
echo Os arquivos serao mantidos em C:\fcdata-api
echo.
echo Tem certeza? (S/N)
choice /C SN /M "Confirmar desinstalacao"
if errorlevel 2 goto MENU
if errorlevel 1 call uninstall_service.bat
goto MENU

:SAIR
cls
echo.
echo Obrigado por usar FC Data API!
echo.
timeout /t 2 /nobreak >nul
exit
