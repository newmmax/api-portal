@echo off
REM ================================================
REM ROLLBACK - REVERTER INSTALAÇÃO
REM ================================================
color 0C
cls

echo ========================================
echo    FC DATA API - ROLLBACK COMPLETO
echo ========================================
echo.
echo ATENÇÃO: Este script vai REMOVER a FC Data API!
echo.
echo Deseja continuar? (S/N)
set /p confirmar=
if /i "%confirmar%" neq "S" (
    echo Rollback cancelado.
    pause
    exit /b 0
)

echo.
echo Iniciando rollback...
echo.

REM 1. Parar serviço
echo [1/6] Parando serviço...
C:\fcdata-api\tools\nssm.exe stop FCDataAPI >nul 2>&1
sc stop FCDataAPI >nul 2>&1
timeout /t 2 >nul
echo [OK] Serviço parado

REM 2. Remover serviço
echo [2/6] Removendo serviço...
C:\fcdata-api\tools\nssm.exe remove FCDataAPI confirm >nul 2>&1
echo [OK] Serviço removido

REM 3. Remover tarefa agendada
echo [3/6] Removendo monitoramento...
schtasks /delete /tn "FC Data API Monitor" /f >nul 2>&1
echo [OK] Tarefa agendada removida

REM 4. Remover regra de firewall
echo [4/6] Removendo regra de firewall...
netsh advfirewall firewall delete rule name="FC Data API Local" >nul 2>&1
echo [OK] Regra de firewall removida

REM 5. Fazer backup dos logs antes de remover
echo [5/6] Salvando logs...
if exist "C:\fcdata-api\logs" (
    xcopy /E /I /Y "C:\fcdata-api\logs" "C:\fcdata-api_backup_logs_%date:~0,2%-%date:~3,2%-%date:~6,4%" >nul
    echo [OK] Logs salvos em backup
)

REM 6. Perguntar se deve remover arquivos
echo.
echo Deseja remover todos os arquivos? (S/N)
echo (Os logs foram salvos em backup)
set /p remover_arquivos=
if /i "%remover_arquivos%" equ "S" (
    echo [6/6] Removendo arquivos...
    rmdir /S /Q "C:\fcdata-api" 2>nul
    echo [OK] Arquivos removidos
) else (
    echo [6/6] Arquivos mantidos em C:\fcdata-api
)

echo.
color 0E
echo ========================================
echo    ROLLBACK CONCLUÍDO
echo ========================================
echo.
echo A FC Data API foi removida do sistema.
echo.
echo IMPORTANTE: Você ainda precisa:
echo   1. Remover as configurações do Apache manualmente
echo   2. Reiniciar o Apache
echo.
pause