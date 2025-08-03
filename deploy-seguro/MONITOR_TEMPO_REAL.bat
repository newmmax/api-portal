@echo off
echo ============================================
echo   FC DATA API - MONITOR RAPIDO
echo ============================================
echo.

:loop
cls
echo ============================================
echo   FC DATA API - MONITOR RAPIDO
echo   Pressione CTRL+C para sair
echo ============================================
echo.
echo Data/Hora: %date% %time%
echo.

echo [STATUS DO SERVICO]
sc query FCDataAPI | findstr "STATE"
echo.

echo [PROCESSO]
wmic process where name="fc-data-api.exe" get ProcessId,WorkingSetSize /format:list | findstr /v "^$" | findstr /v "^No Instance"
echo.

echo [PORTA]
netstat -an | findstr :8089 | findstr LISTENING
echo.

echo [HEALTH CHECK]
curl -s http://localhost:8089/services/api1/health
echo.
echo.

echo [ULTIMAS LINHAS DO LOG]
if exist "C:\fcdata-api\logs\service.log" (
    powershell -Command "Get-Content 'C:\fcdata-api\logs\service.log' -Tail 3" 2>nul
)
echo.

echo [ERROS RECENTES]
if exist "C:\fcdata-api\logs\error.log" (
    for %%A in ("C:\fcdata-api\logs\error.log") do (
        if %%~zA GTR 0 (
            echo ATENCAO: Erros encontrados!
            powershell -Command "Get-Content 'C:\fcdata-api\logs\error.log' -Tail 3" 2>nul
        ) else (
            echo Nenhum erro registrado
        )
    )
)

echo.
echo Atualizando em 10 segundos...
timeout /t 10 /nobreak >nul
goto :loop
