@echo off
REM ===========================================
REM FC Data API - Rotação de Logs
REM ===========================================
REM Executar semanalmente para rotacionar logs

set LOGDIR=C:\fcdata-api\logs
set BACKUPDIR=C:\fcdata-api\backup

echo =========================================
echo FC Data API - Rotacao de Logs
echo =========================================
echo.

REM Criar diretório de backup se não existir
if not exist "%BACKUPDIR%" mkdir "%BACKUPDIR%"

REM Obter timestamp
for /f "tokens=2 delims==" %%a in ('wmic OS Get localdatetime /value') do set "dt=%%a"
set "timestamp=%dt:~0,4%-%dt:~4,2%-%dt:~6,2%_%dt:~8,2%-%dt:~10,2%"

echo Timestamp: %timestamp%
echo.

REM Verificar tamanho dos logs atuais
echo Verificando tamanho dos logs...
for %%A in ("%LOGDIR%\service.log") do (
    if %%~zA GTR 10485760 (
        echo service.log: %%~zA bytes - SERA ROTACIONADO
        move "%LOGDIR%\service.log" "%BACKUPDIR%\service_%timestamp%.log" 2>nul
        echo Arquivo rotacionado para: service_%timestamp%.log
    ) else (
        echo service.log: %%~zA bytes - OK
    )
)

for %%A in ("%LOGDIR%\error.log") do (
    if %%~zA GTR 10485760 (
        echo error.log: %%~zA bytes - SERA ROTACIONADO
        move "%LOGDIR%\error.log" "%BACKUPDIR%\error_%timestamp%.log" 2>nul
        echo Arquivo rotacionado para: error_%timestamp%.log
    ) else (
        echo error.log: %%~zA bytes - OK
    )
)

echo.
echo Limpando backups antigos (mais de 30 dias)...
forfiles /p "%BACKUPDIR%" /m *.log /d -30 /c "cmd /c echo Deletando: @path && del @path" 2>nul

echo.
echo Rotacao concluida!
echo Backups em: %BACKUPDIR%
echo.

REM Se executado manualmente, pausar
if "%1"=="" pause
