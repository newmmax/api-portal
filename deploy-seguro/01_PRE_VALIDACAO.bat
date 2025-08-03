@echo off
REM Força a janela a ficar aberta
if "%1"=="" (
    cmd /k "%~f0" executed
    exit /b
)

echo ============================================
echo   FC DATA API - PRE-VALIDACAO DE DEPLOY
echo   EXECUTAR ANTES DE QUALQUER MUDANCA!
echo ============================================
echo.
echo IMPORTANTE: Execute como ADMINISTRADOR!
echo.
pause

REM Verificar se é admin primeiro
echo Verificando permissoes administrativas...
net session >nul 2>&1
if %errorlevel% NEQ 0 (
    echo.
    echo ========================================
    echo   ERRO: NAO ESTA COMO ADMINISTRADOR!
    echo ========================================
    echo.
    echo Clique com botao direito no arquivo e
    echo escolha "Executar como administrador"
    echo.
    pause
    exit /b 1
)
echo [OK] Executando como Administrador
echo.

REM Criar pasta de logs
if not exist "logs" mkdir logs
set LOGFILE=logs\pre_validacao_%date:~-4,4%%date:~-10,2%%date:~-7,2%_%time:~0,2%%time:~3,2%.log
echo Log sendo salvo em: %LOGFILE%
echo.

echo Iniciando validacao... >> %LOGFILE%
echo ============================================ >> %LOGFILE%
echo.

REM 1. Verificar se o executavel existe
echo [1/10] Verificando executavel compilado...
echo [1/10] Verificando executavel compilado... >> %LOGFILE%
if exist "target\release\fc-data-api.exe" (
    echo [OK] Executavel encontrado
    echo [OK] Executavel encontrado >> %LOGFILE%
    dir "target\release\fc-data-api.exe" | findstr /C:"fc-data-api.exe"
) else (
    echo [ERRO] Executavel nao encontrado!
    echo [ERRO] Executavel nao encontrado! >> %LOGFILE%
    echo.
    echo Compile primeiro com: cargo build --release
    echo.
    pause
    exit /b 1
)
echo.

REM 2. Verificar arquivo .env
echo [2/10] Verificando arquivo .env...
echo [2/10] Verificando arquivo .env... >> %LOGFILE%
if exist ".env" (
    echo [OK] Arquivo .env encontrado
    echo [OK] Arquivo .env encontrado >> %LOGFILE%
) else (
    echo [ERRO] Arquivo .env nao encontrado!
    echo [ERRO] Arquivo .env nao encontrado! >> %LOGFILE%
    echo.
    echo Crie o arquivo .env baseado no .env.example
    echo.
    pause
    exit /b 1
)
echo.

REM 3. Mostrar configuracao atual
echo [3/10] Configuracao atual do .env:
echo =====================================
type .env | findstr "SERVER_PORT"
type .env | findstr "DATABASE_URL" | findstr -v "PASSWORD"
type .env | findstr "API_PREFIX"
echo =====================================
echo.
pause

REM 4. Testar conexao com PostgreSQL
echo [4/10] Testando conexao PostgreSQL...
echo [4/10] Testando conexao PostgreSQL... >> %LOGFILE%
echo Tentando conectar em 10.216.1.16:5432...
powershell -Command "try { $tcp = New-Object System.Net.Sockets.TcpClient; $tcp.Connect('10.216.1.16', 5432); $tcp.Close(); Write-Host '[OK] PostgreSQL acessivel' -ForegroundColor Green } catch { Write-Host '[ERRO] PostgreSQL inacessivel: ' $_.Exception.Message -ForegroundColor Red }"
echo.

REM 5. Verificar porta disponivel
echo [5/10] Verificando porta 8089...
echo [5/10] Verificando porta 8089... >> %LOGFILE%
netstat -an | findstr :8089 | findstr LISTENING >nul
if %errorlevel% == 0 (
    echo [AVISO] Porta 8089 ja esta em uso!
    echo [AVISO] Porta 8089 ja esta em uso! >> %LOGFILE%
    echo.
    echo Processos usando a porta:
    netstat -an | findstr :8089
    echo.
    echo Deseja continuar mesmo assim? (S/N)
    choice /C SN /N
    if errorlevel 2 (
        echo Cancelado pelo usuario
        pause
        exit /b 1
    )
) else (
    echo [OK] Porta 8089 disponivel
    echo [OK] Porta 8089 disponivel >> %LOGFILE%
)
echo.

REM 6. Verificar NSSM
echo [6/10] Verificando NSSM...
echo [6/10] Verificando NSSM... >> %LOGFILE%
where nssm >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] NSSM encontrado no PATH
    echo [OK] NSSM encontrado no PATH >> %LOGFILE%
    where nssm
) else (
    if exist "C:\nssm\win64\nssm.exe" (
        echo [OK] NSSM encontrado em C:\nssm
        echo [OK] NSSM encontrado em C:\nssm >> %LOGFILE%
    ) else (
        echo [ERRO] NSSM nao encontrado!
        echo [ERRO] NSSM nao encontrado! >> %LOGFILE%
        echo.
        echo Baixe NSSM de: https://nssm.cc
        echo Extraia para C:\nssm\
        echo.
        pause
        exit /b 1
    )
)
echo.

REM 7. Verificar Apache
echo [7/10] Verificando Apache...
echo [7/10] Verificando Apache... >> %LOGFILE%
if exist "C:\XAMPP\apache\bin\httpd.exe" (
    echo [OK] Apache XAMPP encontrado
    echo [OK] Apache XAMPP encontrado >> %LOGFILE%
) else (
    echo [AVISO] Apache nao encontrado no local esperado
    echo [AVISO] Apache nao encontrado >> %LOGFILE%
    echo Voce precisara configurar o proxy manualmente
)
echo.

REM 8. Verificar espaco em disco
echo [8/10] Verificando espaco em disco...
echo [8/10] Verificando espaco em disco... >> %LOGFILE%
for /f "tokens=3" %%a in ('dir C:\ /-c 2^>nul ^| findstr /c:"bytes free"') do set FREE_SPACE=%%a
echo Espaco livre: %FREE_SPACE% bytes
echo.

REM 9. Testar executavel localmente
echo [9/10] Testando executavel localmente...
echo [9/10] Testando executavel... >> %LOGFILE%
echo.
echo Vou iniciar a API por 10 segundos para teste...
echo Se aparecer algum erro, anote!
echo.
pause

echo Iniciando API...
cd /d "%~dp0\.."
start /B target\release\fc-data-api.exe
echo Aguardando 10 segundos...
timeout /t 10 /nobreak

echo.
echo Testando health check...
curl -s http://localhost:8089/services/api1/health
if %errorlevel% == 0 (
    echo.
    echo [OK] API respondendo corretamente!
    echo [OK] API respondendo! >> %LOGFILE%
) else (
    echo.
    echo [ERRO] API nao respondeu ao teste!
    echo [ERRO] API nao respondeu! >> %LOGFILE%
)

echo.
echo Parando API de teste...
taskkill /F /IM fc-data-api.exe >nul 2>&1
timeout /t 2 /nobreak >nul
echo.

REM 10. Verificar servico existente
echo [10/10] Verificando servicos existentes...
echo [10/10] Verificando servicos... >> %LOGFILE%
sc query FCDataAPI >nul 2>&1
if %errorlevel% == 0 (
    echo [AVISO] Servico FCDataAPI ja existe!
    echo [AVISO] Servico ja existe! >> %LOGFILE%
    echo Sera necessario remove-lo antes de reinstalar
) else (
    echo [OK] Nenhum servico conflitante
    echo [OK] Nenhum servico conflitante >> %LOGFILE%
)
echo.

echo ============================================
echo   RESUMO DA VALIDACAO
echo ============================================
type %LOGFILE% | findstr /C:"[OK]" /C:"[ERRO]" /C:"[AVISO]"
echo.
echo ============================================
echo   FIM DA VALIDACAO
echo ============================================
echo.
echo Se tudo estiver OK, execute:
echo 02_BACKUP_ATUAL.bat
echo.
echo Log completo salvo em: %LOGFILE%
echo.
echo Pressione qualquer tecla para sair...
pause >nul
