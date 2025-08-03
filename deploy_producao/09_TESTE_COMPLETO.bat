@echo off
REM ================================================
REM PASSO 9: TESTE COMPLETO DO SISTEMA
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - TESTE COMPLETO
echo ========================================
echo.

set TESTES_OK=0
set TOTAL_TESTES=0

REM Teste 1: Servico Windows
echo [1/5] Verificando servico Windows...
set /a TOTAL_TESTES+=1
sc query FCDataAPI | findstr /i "running" >nul
if %errorlevel% eq 0 (
    echo [OK] Servico esta rodando
    set /a TESTES_OK+=1
) else (
    echo [FALHA] Servico nao esta rodando
    sc query FCDataAPI
)
echo.

REM Teste 2: API direta
echo [2/5] Testando API diretamente...
set /a TOTAL_TESTES+=1
powershell -Command "(New-Object Net.WebClient).DownloadString('http://127.0.0.1:8089/services/api1/health')" >nul 2>&1
if %errorlevel% eq 0 (
    echo [OK] API respondendo em http://127.0.0.1:8089
    set /a TESTES_OK+=1
) else (
    echo [FALHA] API nao responde diretamente
)
echo.

REM Teste 3: Proxy (se configurado)
echo [3/5] Testando via servidor web...
set /a TOTAL_TESTES+=1
powershell -Command "(New-Object Net.WebClient).DownloadString('http://localhost/services/api1/health')" >nul 2>&1
if %errorlevel% eq 0 (
    echo [OK] Proxy funcionando
    set /a TESTES_OK+=1
) else (
    echo [AVISO] Proxy ainda nao configurado ou nao funcionando
)
echo.

REM Teste 4: Login
echo [4/5] Testando autenticacao...
set /a TOTAL_TESTES+=1
echo {"username":"admin_prod","password":"Pr0duc@0_FC_2025!Art3s@n@l"} > temp_login.json
powershell -Command "$body = Get-Content temp_login.json; Invoke-RestMethod -Uri 'http://127.0.0.1:8089/services/api1/auth/login' -Method POST -Body $body -ContentType 'application/json'" >nul 2>&1
if %errorlevel% eq 0 (
    echo [OK] Autenticacao funcionando
    set /a TESTES_OK+=1
) else (
    echo [FALHA] Autenticacao nao funcionou
)
del temp_login.json >nul 2>&1
echo.

REM Teste 5: Logs
echo [5/5] Verificando logs...
set /a TOTAL_TESTES+=1
if exist "C:\fcdata-api\logs\service.log" (
    echo [OK] Logs sendo gerados
    set /a TESTES_OK+=1
) else (
    echo [AVISO] Arquivo de log ainda nao criado
)
echo.

REM Resultado final
echo ========================================
if %TESTES_OK% equ %TOTAL_TESTES% (
    color 0A
    echo    TODOS OS TESTES PASSARAM!
    echo    API ESTA PRONTA PARA USO!
) else (
    color 0E
    echo    RESULTADO: %TESTES_OK% de %TOTAL_TESTES% testes OK
    echo.
    echo    Verifique os testes que falharam.
)
echo ========================================
echo.

echo URLs da API:
echo   - Direto: http://127.0.0.1:8089/services/api1
echo   - Via Proxy: http://localhost/services/api1
echo   - Producao: https://conexao.artesanalfarmacia.com.br/services/api1
echo.
echo Credenciais:
echo   - Username: admin_prod
echo   - Password: Pr0duc@0_FC_2025!Art3s@n@l
echo.
echo Logs em: C:\fcdata-api\logs\
echo.

if %TESTES_OK% equ %TOTAL_TESTES% (
    echo Proximo passo: Execute 10_MONITORAMENTO.bat
) else (
    echo Corrija os problemas antes de prosseguir.
)
echo.
pause