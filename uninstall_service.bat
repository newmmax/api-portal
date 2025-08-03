@echo off
REM ===========================================
REM FC Data API - Desinstalação do Serviço
REM ===========================================

echo =========================================
echo FC Data API - Desinstalacao do Servico
echo =========================================
echo.

REM Verificar se está rodando como admin
net session >nul 2>&1
if %errorLevel% NEQ 0 (
    echo ERRO: Este script precisa ser executado como Administrador!
    echo.
    pause
    exit /b 1
)

set SERVICE_NAME=FCDataAPI
set INSTALL_DIR=C:\fcdata-api

echo Este script ira:
echo - Parar o servico %SERVICE_NAME%
echo - Remover o servico do Windows
echo - Manter os arquivos em %INSTALL_DIR%
echo.
echo Pressione Ctrl+C para cancelar ou
pause

echo.
echo [1/4] Parando servico...
nssm stop %SERVICE_NAME% >nul 2>&1
sc stop %SERVICE_NAME% >nul 2>&1
timeout /t 3 /nobreak >nul
echo Servico parado!

echo.
echo [2/4] Removendo servico...
nssm remove %SERVICE_NAME% confirm >nul 2>&1
echo Servico removido!

echo.
echo [3/4] Removendo tarefas agendadas...
schtasks /delete /tn "FCDataAPI_HealthCheck" /f >nul 2>&1
schtasks /delete /tn "FCDataAPI_LogRotation" /f >nul 2>&1
schtasks /delete /tn "FCDataAPI_Backup" /f >nul 2>&1
echo Tarefas removidas!

echo.
echo [4/4] Limpeza concluida!
echo.
echo =========================================
echo Desinstalacao concluida com sucesso!
echo =========================================
echo.
echo Arquivos mantidos em: %INSTALL_DIR%
echo Para remover completamente, delete a pasta manualmente.
echo.
pause
