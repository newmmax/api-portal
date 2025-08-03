@echo off
REM ===========================================
REM FC Data API - Configurar Tarefas Agendadas
REM ===========================================
REM Configura health check, rotacao de logs e backup automaticos

echo =============================================
echo FC Data API - Configuracao de Tarefas Agendadas
echo =============================================
echo.

REM Verificar se está rodando como admin
net session >nul 2>&1
if %errorLevel% NEQ 0 (
    echo ERRO: Este script precisa ser executado como Administrador!
    echo.
    pause
    exit /b 1
)

set INSTALL_DIR=C:\fcdata-api

echo Este script ira criar as seguintes tarefas agendadas:
echo.
echo 1. Health Check - A cada 5 minutos
echo    Verifica se a API esta respondendo e reinicia se necessario
echo.
echo 2. Rotacao de Logs - Semanalmente (Domingos 00:00)
echo    Rotaciona logs grandes e limpa logs antigos
echo.
echo 3. Backup Diario - Todos os dias as 02:00
echo    Faz backup completo do sistema
echo.
pause

REM 1. Remover tarefas antigas se existirem
echo.
echo [1/4] Removendo tarefas antigas (se existirem)...
schtasks /delete /tn "FCDataAPI_HealthCheck" /f >nul 2>&1
schtasks /delete /tn "FCDataAPI_LogRotation" /f >nul 2>&1
schtasks /delete /tn "FCDataAPI_Backup" /f >nul 2>&1
echo Tarefas antigas removidas!

REM 2. Criar tarefa de Health Check
echo.
echo [2/4] Criando tarefa de Health Check...
schtasks /create ^
    /tn "FCDataAPI_HealthCheck" ^
    /tr "powershell.exe -ExecutionPolicy Bypass -WindowStyle Hidden -File \"%INSTALL_DIR%\health_check.ps1\"" ^
    /sc minute ^
    /mo 5 ^
    /ru SYSTEM ^
    /rl HIGHEST ^
    /f

if %errorLevel% EQU 0 (
    echo Health Check configurado com sucesso!
) else (
    echo ERRO ao criar tarefa de Health Check!
)

REM 3. Criar tarefa de Rotação de Logs
echo.
echo [3/4] Criando tarefa de Rotacao de Logs...
schtasks /create ^
    /tn "FCDataAPI_LogRotation" ^
    /tr "\"%INSTALL_DIR%\rotate_logs.bat\" auto" ^
    /sc weekly ^
    /d SUN ^
    /st 00:00 ^
    /ru SYSTEM ^
    /rl HIGHEST ^
    /f

if %errorLevel% EQU 0 (
    echo Rotacao de Logs configurada com sucesso!
) else (
    echo ERRO ao criar tarefa de Rotacao de Logs!
)

REM 4. Criar tarefa de Backup
echo.
echo [4/4] Criando tarefa de Backup Diario...
schtasks /create ^
    /tn "FCDataAPI_Backup" ^
    /tr "\"%INSTALL_DIR%\backup_system.bat\" auto" ^
    /sc daily ^
    /st 02:00 ^
    /ru SYSTEM ^
    /rl HIGHEST ^
    /f

if %errorLevel% EQU 0 (
    echo Backup Diario configurado com sucesso!
) else (
    echo ERRO ao criar tarefa de Backup!
    echo Nota: Verifique se o caminho do backup (D:\Backups) existe
)

REM Mostrar tarefas criadas
echo.
echo =============================================
echo Tarefas agendadas criadas:
echo =============================================
echo.
schtasks /query /tn "FCDataAPI_HealthCheck" /fo LIST /v | findstr "Nome Estado Proxima"
echo.
schtasks /query /tn "FCDataAPI_LogRotation" /fo LIST /v | findstr "Nome Estado Proxima"
echo.
schtasks /query /tn "FCDataAPI_Backup" /fo LIST /v | findstr "Nome Estado Proxima"

echo.
echo =============================================
echo Configuracao concluida!
echo =============================================
echo.
echo Para verificar/editar as tarefas:
echo - Abra o Agendador de Tarefas (taskschd.msc)
echo - Procure por tarefas comecando com "FCDataAPI_"
echo.
echo Para executar uma tarefa manualmente:
echo - schtasks /run /tn "FCDataAPI_HealthCheck"
echo - schtasks /run /tn "FCDataAPI_LogRotation"
echo - schtasks /run /tn "FCDataAPI_Backup"
echo.
pause
