@echo off
echo ============================================
echo   FC DATA API - VALIDACAO FINAL
echo   CONFIRMA QUE TUDO ESTA FUNCIONANDO
echo ============================================
echo.

set ERRORS=0

echo [1/10] Verificando servico Windows...
echo -------------------------------------
sc query FCDataAPI | findstr "RUNNING" >nul
if %errorlevel% == 0 (
    echo [OK] Servico rodando
    sc query FCDataAPI | findstr "STATE"
) else (
    echo [ERRO] Servico nao esta rodando!
    set /a ERRORS+=1
)
echo.

echo [2/10] Verificando processo...
echo ------------------------------
tasklist | findstr fc-data-api.exe >nul
if %errorlevel% == 0 (
    echo [OK] Processo fc-data-api.exe ativo
    wmic process where name="fc-data-api.exe" get ProcessId,WorkingSetSize,PageFileUsage /format:list | findstr /v "^$"
) else (
    echo [ERRO] Processo nao encontrado!
    set /a ERRORS+=1
)
echo.

echo [3/10] Testando Health Check local...
echo -------------------------------------
curl -s http://localhost:8089/services/api1/health > temp_health.json 2>nul
if %errorlevel% == 0 (
    echo [OK] Health check respondeu
    type temp_health.json
    del temp_health.json
) else (
    echo [ERRO] Health check falhou!
    set /a ERRORS+=1
)
echo.

echo [4/10] Testando autenticacao...
echo -------------------------------
echo Fazendo login...
curl -s -X POST http://localhost:8089/services/api1/auth/login ^
  -H "Content-Type: application/json" ^
  -d "{\"username\":\"admin\",\"password\":\"ArtesanalFC2025!\"}" > temp_login.json 2>nul

if %errorlevel% == 0 (
    type temp_login.json | findstr "token" >nul
    if %errorlevel% == 0 (
        echo [OK] Login funcionando
        for /f "delims=" %%i in ('powershell -Command "(Get-Content temp_login.json | ConvertFrom-Json).token"') do set TOKEN=%%i
    ) else (
        echo [ERRO] Login retornou erro!
        type temp_login.json
        set /a ERRORS+=1
    )
) else (
    echo [ERRO] Nao foi possivel fazer login!
    set /a ERRORS+=1
)
del temp_login.json 2>nul
echo.

echo [5/10] Testando consulta de dados...
echo ------------------------------------
if defined TOKEN (
    echo Consultando vendas...
    curl -s -X GET "http://localhost:8089/services/api1/data/vendas?limite=1" ^
      -H "Authorization: Bearer %TOKEN%" > temp_vendas.json 2>nul
    
    if %errorlevel% == 0 (
        type temp_vendas.json | findstr "error" >nul
        if %errorlevel% == 1 (
            echo [OK] Consulta funcionando
            echo Primeira linha de resposta:
            powershell -Command "Get-Content temp_vendas.json | Select-Object -First 1"
        ) else (
            echo [ERRO] Consulta retornou erro!
            type temp_vendas.json
            set /a ERRORS+=1
        )
    ) else (
        echo [ERRO] Consulta falhou!
        set /a ERRORS+=1
    )
    del temp_vendas.json 2>nul
) else (
    echo [AVISO] Pulando teste - sem token
)
echo.

echo [6/10] Verificando logs...
echo --------------------------
if exist "C:\fcdata-api\logs\service.log" (
    echo [OK] Log de servico existe
    echo Ultimas 5 linhas:
    powershell -Command "Get-Content 'C:\fcdata-api\logs\service.log' -Tail 5"
) else (
    echo [AVISO] Log de servico nao encontrado
)
echo.
if exist "C:\fcdata-api\logs\error.log" (
    for %%A in ("C:\fcdata-api\logs\error.log") do (
        if %%~zA GTR 0 (
            echo [AVISO] Log de erro tem conteudo!
            echo Ultimas 5 linhas:
            powershell -Command "Get-Content 'C:\fcdata-api\logs\error.log' -Tail 5"
        ) else (
            echo [OK] Log de erro vazio
        )
    )
)
echo.

echo [7/10] Verificando configuracao Apache...
echo -----------------------------------------
findstr /C:"ProxyPass /services/api1" "C:\XAMPP\apache\conf\extra\httpd-vhosts.conf" >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Proxy configurado no Apache
    findstr /C:"ProxyPass /services/api1" "C:\XAMPP\apache\conf\extra\httpd-vhosts.conf"
) else (
    echo [AVISO] Proxy nao configurado no Apache
    echo Execute o passo 8 do deploy se necessario
)
echo.

echo [8/10] Verificando conectividade PostgreSQL...
echo ----------------------------------------------
powershell -Command "try { $tcp = New-Object System.Net.Sockets.TcpClient; $tcp.Connect('10.216.1.16', 5432); $tcp.Close(); Write-Host '[OK] PostgreSQL acessivel' } catch { Write-Host '[ERRO] PostgreSQL inacessivel' }"
echo.

echo [9/10] Verificando recuperacao automatica...
echo --------------------------------------------
sc failure FCDataAPI | findstr "RESTART" >nul
if %errorlevel% == 0 (
    echo [OK] Recuperacao automatica configurada
) else (
    echo [AVISO] Configurando recuperacao automatica...
    sc failure FCDataAPI reset= 86400 actions= restart/60000/restart/60000/restart/60000
)
echo.

echo [10/10] Gerando relatorio de saude...
echo -------------------------------------
(
echo FC Data API - Relatorio de Validacao
echo Data: %date% %time%
echo.
echo Servico: FCDataAPI
sc query FCDataAPI
echo.
echo Processo:
wmic process where name="fc-data-api.exe" get ProcessId,WorkingSetSize,PageFileUsage /format:list
echo.
echo Portas em uso:
netstat -an | findstr :8089
echo.
echo Arquivos em C:\fcdata-api:
dir /B C:\fcdata-api
echo.
echo Total de Erros: %ERRORS%
) > validacao_report_%date:~-4,4%%date:~-10,2%%date:~-7,2%.txt

echo [OK] Relatorio salvo em validacao_report_%date:~-4,4%%date:~-10,2%%date:~-7,2%.txt
echo.

echo ============================================
if %ERRORS% EQU 0 (
    echo   VALIDACAO CONCLUIDA - TUDO OK!
    echo ============================================
    echo.
    echo A API esta funcionando perfeitamente!
    echo.
    echo URLs de acesso:
    echo - Local: http://localhost:8089/services/api1
    echo - HTTPS: https://conexao.artesanalfarmacia.com.br/services/api1
) else (
    echo   VALIDACAO CONCLUIDA - ENCONTRADOS %ERRORS% ERROS!
    echo ============================================
    echo.
    echo ATENCAO: Foram encontrados problemas!
    echo Verifique os erros acima e:
    echo 1. Consulte os logs em C:\fcdata-api\logs\
    echo 2. Use o script ROLLBACK se necessario
)
echo.
pause
