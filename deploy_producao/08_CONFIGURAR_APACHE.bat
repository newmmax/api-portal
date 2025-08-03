@echo off
REM ================================================
REM PASSO 8: CONFIGURAR APACHE PROXY
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - CONFIGURAR APACHE
echo ========================================
echo.
echo ATENÇÃO: Este script vai modificar a configuração do Apache!
echo.
echo Certifique-se de ter um backup antes de continuar.
echo.
pause

REM Localizar Apache
set APACHE_PATH=
if exist "C:\xampp\apache" (
    set APACHE_PATH=C:\xampp\apache
) else if exist "C:\Apache24" (
    set APACHE_PATH=C:\Apache24
) else if exist "C:\Program Files\Apache Software Foundation\Apache2.4" (
    set APACHE_PATH=C:\Program Files\Apache Software Foundation\Apache2.4
) else (
    echo [ERRO] Apache não encontrado!
    echo Por favor, configure manualmente.
    pause
    exit /b 1
)

echo Apache encontrado em: %APACHE_PATH%
echo.

REM Fazer backup
echo Fazendo backup das configurações...
copy "%APACHE_PATH%\conf\httpd.conf" "%APACHE_PATH%\conf\httpd.conf.bak_%date:~0,2%-%date:~3,2%-%date:~6,4%" >nul
echo [OK] Backup criado
echo.

echo ========================================
echo    CONFIGURAÇÃO MANUAL NECESSÁRIA
echo ========================================
echo.
echo Por segurança, você deve editar manualmente:
echo.
echo 1. Abra: %APACHE_PATH%\conf\httpd.conf
echo.
echo 2. Encontre e descomente (remova #) estas linhas:
echo    LoadModule proxy_module modules/mod_proxy.so
echo    LoadModule proxy_http_module modules/mod_proxy_http.so
echo    LoadModule headers_module modules/mod_headers.so
echo.
echo 3. No final do arquivo, adicione:
echo.
echo    # FC Data API Proxy
echo    ^<Location /services/api1^>
echo        ProxyPreserveHost On
echo        ProxyPass http://127.0.0.1:8089/services/api1
echo        ProxyPassReverse http://127.0.0.1:8089/services/api1
echo        ProxyTimeout 300
echo        RequestHeader set X-Forwarded-Proto "https"
echo        RequestHeader set X-Forwarded-For "%%{REMOTE_ADDR}s"
echo    ^</Location^>
echo.
echo 4. Salve o arquivo
echo.
echo 5. Teste a configuração:
echo    %APACHE_PATH%\bin\httpd -t
echo.
echo 6. Reinicie o Apache:
echo    %APACHE_PATH%\bin\httpd -k restart
echo.
echo Após configurar, execute 09_TESTE_COMPLETO.bat
echo.
pause