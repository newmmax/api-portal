@echo off
REM ================================================
REM PASSO 10: CONFIGURAR MONITORAMENTO
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - MONITORAMENTO
echo ========================================
echo.

REM Criar script de monitoramento
echo Criando script de monitoramento...
(
echo @echo off
echo :loop
echo curl -s http://127.0.0.1:8089/services/api1/health ^>nul 2^>^&1
echo if %%errorlevel%% neq 0 ^(
echo     echo %%date%% %%time%% - API DOWN ^>^> C:\fcdata-api\logs\monitor.log
echo     C:\fcdata-api\tools\nssm.exe restart FCDataAPI
echo     echo %%date%% %%time%% - Tentativa de restart ^>^> C:\fcdata-api\logs\monitor.log
echo ^)
echo timeout /t 60 ^>nul
echo goto loop
) > C:\fcdata-api\monitor.bat

echo [OK] Script de monitoramento criado
echo.

REM Criar tarefa agendada
echo Criando tarefa agendada...
schtasks /create /tn "FC Data API Monitor" /tr "C:\fcdata-api\monitor.bat" /sc onstart /ru SYSTEM /f >nul 2>&1

if %errorlevel% eq 0 (
    echo [OK] Tarefa agendada criada
) else (
    echo [AVISO] Não foi possível criar tarefa agendada
    echo Configure manualmente no Agendador de Tarefas
)

echo.
echo ========================================
echo    DEPLOY COMPLETO!
echo ========================================
echo.
echo A FC Data API está instalada e funcionando!
echo.
echo Resumo da instalação:
echo   - Serviço: FCDataAPI
echo   - Porta: 8089 (apenas local)
echo   - Logs: C:\fcdata-api\logs\
echo   - URL: https://conexao.artesanalfarmacia.com.br/services/api1
echo.
echo Comandos úteis:
echo   - Status: C:\fcdata-api\tools\nssm.exe status FCDataAPI
echo   - Parar: C:\fcdata-api\tools\nssm.exe stop FCDataAPI
echo   - Iniciar: C:\fcdata-api\tools\nssm.exe start FCDataAPI
echo   - Logs: type C:\fcdata-api\logs\service.log
echo.
echo IMPORTANTE: Mude a senha do admin após o primeiro acesso!
echo.
pause

REM Abrir logs
start notepad C:\fcdata-api\logs\service.log