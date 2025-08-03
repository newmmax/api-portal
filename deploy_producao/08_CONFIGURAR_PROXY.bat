@echo off
REM ================================================
REM PASSO 8: CONFIGURAR IIS/APACHE PROXY
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - CONFIGURAR PROXY
echo ========================================
echo.

REM Detectar IIS
sc query W3SVC >nul 2>&1
if %errorlevel% eq 0 (
    echo [INFO] IIS detectado no servidor
    echo.
    echo ========================================
    echo    CONFIGURACAO MANUAL IIS
    echo ========================================
    echo.
    echo 1. Instale os modulos necessarios:
    echo    - Application Request Routing (ARR)
    echo    - URL Rewrite
    echo.
    echo 2. Abra o IIS Manager
    echo.
    echo 3. Selecione o site principal
    echo.
    echo 4. Configure URL Rewrite com a regra:
    echo    Pattern: ^services/api1/(.*)$
    echo    Rewrite URL: http://127.0.0.1:8089/services/api1/{R:1}
    echo.
    echo 5. Configure ARR Proxy Settings:
    echo    - Enable proxy
    echo    - Preserve Host Header: ON
    echo.
    goto fim
)

REM Verificar Apache
set APACHE_FOUND=0
if exist "C:\xampp\apache" (
    set APACHE_PATH=C:\xampp\apache
    set APACHE_FOUND=1
) else if exist "C:\Apache24" (
    set APACHE_PATH=C:\Apache24
    set APACHE_FOUND=1
)

if %APACHE_FOUND% equ 1 (
    echo [INFO] Apache encontrado em: %APACHE_PATH%
    echo.
    echo Fazendo backup...
    copy "%APACHE_PATH%\conf\httpd.conf" "%APACHE_PATH%\conf\httpd.conf.bak" >nul
    echo [OK] Backup criado
    echo.
    echo ========================================
    echo    CONFIGURACAO MANUAL APACHE
    echo ========================================
    echo.
    echo 1. Edite: %APACHE_PATH%\conf\httpd.conf
    echo.
    echo 2. Descomente estas linhas:
    echo    LoadModule proxy_module modules/mod_proxy.so
    echo    LoadModule proxy_http_module modules/mod_proxy_http.so
    echo    LoadModule headers_module modules/mod_headers.so
    echo.
    echo 3. Adicione no final:
    echo.
    echo    # FC Data API Proxy
    echo    ^<Location /services/api1^>
    echo        ProxyPreserveHost On
    echo        ProxyPass http://127.0.0.1:8089/services/api1
    echo        ProxyPassReverse http://127.0.0.1:8089/services/api1
    echo        ProxyTimeout 300
    echo    ^</Location^>
    echo.
    echo 4. Teste: %APACHE_PATH%\bin\httpd -t
    echo 5. Reinicie: %APACHE_PATH%\bin\httpd -k restart
) else (
    echo [AVISO] Nenhum servidor web detectado automaticamente
    echo Configure manualmente o proxy reverso no seu servidor web
)

:fim
echo.
echo Apos configurar, execute 09_TESTE_COMPLETO.bat
echo.
pause