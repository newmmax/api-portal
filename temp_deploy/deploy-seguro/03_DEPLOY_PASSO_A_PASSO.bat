@echo off
echo ============================================
echo   FC DATA API - DEPLOY PASSO A PASSO
echo   MODO SEGURO COM VALIDACOES
echo ============================================
echo.
echo IMPORTANTE: Este script fara pausas entre cada
echo etapa para validacao. NAO PULE AS VALIDACOES!
echo.
pause

REM Verificar se executou pre-validacao
echo.
echo Voce executou 01_PRE_VALIDACAO.bat? (S/N)
choice /C SN /N
if errorlevel 2 (
    echo Por favor, execute primeiro!
    pause
    exit /b 1
)

REM Verificar se fez backup
echo.
echo Voce executou 02_BACKUP_ATUAL.bat? (S/N)
choice /C SN /N
if errorlevel 2 (
    echo Por favor, faca o backup primeiro!
    pause
    exit /b 1
)

echo.
echo ============================================
echo   INICIANDO DEPLOY...
echo ============================================
echo.

REM PASSO 1: Parar servico existente (se houver)
echo [PASSO 1/8] Parando servico existente...
echo -------------------------------------------
sc query FCDataAPI >nul 2>&1
if %errorlevel% == 0 (
    echo Servico encontrado, parando...
    nssm stop FCDataAPI
    timeout /t 5 /nobreak
    echo [OK] Servico parado
) else (
    echo [OK] Nenhum servico para parar
)
echo.
pause

REM PASSO 2: Criar estrutura de diretorios
echo [PASSO 2/8] Criando estrutura de diretorios...
echo ---------------------------------------------
if not exist "C:\fcdata-api" (
    mkdir "C:\fcdata-api"
    echo [OK] Diretorio criado: C:\fcdata-api
) else (
    echo [OK] Diretorio ja existe: C:\fcdata-api
)

if not exist "C:\fcdata-api\logs" (
    mkdir "C:\fcdata-api\logs"
    echo [OK] Diretorio criado: C:\fcdata-api\logs
)

if not exist "C:\fcdata-api\backup" (
    mkdir "C:\fcdata-api\backup"
    echo [OK] Diretorio criado: C:\fcdata-api\backup
)
echo.
pause

REM PASSO 3: Copiar arquivos
echo [PASSO 3/8] Copiando arquivos...
echo --------------------------------
echo Copiando executavel...
copy "target\release\fc-data-api.exe" "C:\fcdata-api\" /Y
if %errorlevel% == 0 (
    echo [OK] Executavel copiado
) else (
    echo [ERRO] Falha ao copiar executavel!
    pause
    exit /b 1
)

echo Copiando .env...
copy ".env" "C:\fcdata-api\" /Y
if %errorlevel% == 0 (
    echo [OK] .env copiado
) else (
    echo [ERRO] Falha ao copiar .env!
    pause
    exit /b 1
)

echo Copiando documentacao...
copy "README.md" "C:\fcdata-api\" /Y >nul 2>&1
echo.
pause

REM PASSO 4: Testar executavel antes de criar servico
echo [PASSO 4/8] Testando executavel...
echo ----------------------------------
echo Iniciando teste de 10 segundos...
cd /d C:\fcdata-api
start /B fc-data-api.exe
timeout /t 10 /nobreak

echo.
echo Testando health check...
curl -s http://localhost:8089/services/api1/health
if %errorlevel% == 0 (
    echo.
    echo [OK] API respondendo corretamente!
) else (
    echo.
    echo [ERRO] API nao respondeu!
    taskkill /F /IM fc-data-api.exe >nul 2>&1
    pause
    exit /b 1
)

echo.
echo Parando teste...
taskkill /F /IM fc-data-api.exe >nul 2>&1
timeout /t 2 /nobreak >nul
echo [OK] Teste concluido com sucesso
cd /d %~dp0
echo.
pause

REM PASSO 5: Remover servico antigo (se existir)
echo [PASSO 5/8] Removendo servico antigo...
echo ---------------------------------------
sc query FCDataAPI >nul 2>&1
if %errorlevel% == 0 (
    echo Removendo servico existente...
    nssm remove FCDataAPI confirm
    timeout /t 3 /nobreak >nul
    echo [OK] Servico removido
) else (
    echo [OK] Nenhum servico para remover
)
echo.
pause

REM PASSO 6: Instalar novo servico
echo [PASSO 6/8] Instalando servico Windows...
echo -----------------------------------------
echo Usando NSSM para criar servico...

REM Tentar usar NSSM do PATH primeiro
where nssm >nul 2>&1
if %errorlevel% == 0 (
    set NSSM_CMD=nssm
) else (
    if exist "C:\nssm\win64\nssm.exe" (
        set NSSM_CMD=C:\nssm\win64\nssm.exe
    ) else (
        echo [ERRO] NSSM nao encontrado!
        pause
        exit /b 1
    )
)

echo Criando servico...
%NSSM_CMD% install FCDataAPI "C:\fcdata-api\fc-data-api.exe"

echo Configurando servico...
%NSSM_CMD% set FCDataAPI AppDirectory "C:\fcdata-api"
%NSSM_CMD% set FCDataAPI DisplayName "FC Data API Service"
%NSSM_CMD% set FCDataAPI Description "API REST para consulta de dados FC PostgreSQL"
%NSSM_CMD% set FCDataAPI Start SERVICE_AUTO_START

echo Configurando recuperacao de falhas...
%NSSM_CMD% set FCDataAPI AppThrottle 1500
%NSSM_CMD% set FCDataAPI AppExit Default Restart
%NSSM_CMD% set FCDataAPI AppRestartDelay 5000

echo Configurando logs...
%NSSM_CMD% set FCDataAPI AppStdout "C:\fcdata-api\logs\service.log"
%NSSM_CMD% set FCDataAPI AppStderr "C:\fcdata-api\logs\error.log"
%NSSM_CMD% set FCDataAPI AppRotateFiles 1
%NSSM_CMD% set FCDataAPI AppRotateOnline 1
%NSSM_CMD% set FCDataAPI AppRotateBytes 10485760

echo.
echo [OK] Servico instalado e configurado
echo.
pause

REM PASSO 7: Iniciar servico
echo [PASSO 7/8] Iniciando servico...
echo --------------------------------
%NSSM_CMD% start FCDataAPI
timeout /t 5 /nobreak

echo.
echo Verificando status...
sc query FCDataAPI | findstr "RUNNING"
if %errorlevel% == 0 (
    echo [OK] Servico rodando!
) else (
    echo [ERRO] Servico nao iniciou!
    echo.
    echo Verificando logs...
    type "C:\fcdata-api\logs\error.log" 2>nul
    pause
    exit /b 1
)

echo.
echo Testando API via servico...
curl -s http://localhost:8089/services/api1/health
if %errorlevel% == 0 (
    echo.
    echo [OK] API respondendo via servico!
) else (
    echo.
    echo [ERRO] API nao respondeu!
    pause
    exit /b 1
)
echo.
pause

REM PASSO 8: Configurar Apache (opcional)
echo [PASSO 8/8] Configurar Apache Proxy...
echo --------------------------------------
echo.
echo Deseja configurar o Apache agora? (S/N)
echo (Pode ser feito manualmente depois)
choice /C SN /N
if errorlevel 2 goto :fim

echo.
echo INSTRUCOES PARA APACHE:
echo.
echo 1. Abra: C:\XAMPP\apache\conf\extra\httpd-vhosts.conf
echo 2. Dentro do VirtualHost HTTPS, adicione:
echo.
echo    # Proxy para FC Data API
echo    ProxyPass /services/api1 http://localhost:8089/services/api1
echo    ProxyPassReverse /services/api1 http://localhost:8089/services/api1
echo.
echo 3. Salve o arquivo
echo 4. Reinicie o Apache
echo.
echo Pressione qualquer tecla quando terminar...
pause >nul

echo.
echo Reiniciando Apache...
C:\XAMPP\apache\bin\httpd.exe -k restart
timeout /t 5 /nobreak

echo.
echo Testando acesso via HTTPS...
echo (Pode falhar se o certificado nao for valido localmente)
curl -k https://localhost/services/api1/health 2>nul
echo.

:fim
echo.
echo ============================================
echo   DEPLOY CONCLUIDO COM SUCESSO!
echo ============================================
echo.
echo Status Final:
echo - Servico: FCDataAPI instalado e rodando
echo - API Local: http://localhost:8089/services/api1
echo - Logs em: C:\fcdata-api\logs\
echo.
echo IMPORTANTE:
echo - Execute 04_VALIDACAO_FINAL.bat para confirmar
echo - Em caso de problemas, use o script ROLLBACK
echo   do diretorio de backup criado anteriormente
echo.
pause
