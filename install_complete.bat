@echo off
REM ===========================================
REM FC Data API - Instalação Completa Windows
REM ===========================================
REM Este script instala a API como serviço Windows resiliente
REM Executar como ADMINISTRADOR

echo ============================================
echo FC Data API - Instalacao Completa no Windows
echo ============================================
echo.

REM Verificar se está rodando como admin
net session >nul 2>&1
if %errorLevel% NEQ 0 (
    echo ERRO: Este script precisa ser executado como Administrador!
    echo Clique com botao direito e selecione "Executar como administrador"
    echo.
    pause
    exit /b 1
)

REM Verificar se NSSM está disponível
where nssm >nul 2>&1
if %errorLevel% NEQ 0 (
    echo ERRO: NSSM nao encontrado!
    echo.
    echo Baixe o NSSM em: https://nssm.cc/download
    echo Extraia para C:\nssm\ e adicione ao PATH do sistema.
    echo.
    pause
    exit /b 1
)

REM Definir variáveis
set SERVICE_NAME=FCDataAPI
set INSTALL_DIR=C:\fcdata-api
set SOURCE_DIR=%~dp0

echo.
echo [1/8] Criando estrutura de diretorios...
echo ----------------------------------------
mkdir "%INSTALL_DIR%" 2>nul
mkdir "%INSTALL_DIR%\logs" 2>nul
mkdir "%INSTALL_DIR%\backup" 2>nul
echo Diretorios criados!

echo.
echo [2/8] Compilando versao Release...
echo ----------------------------------
cd /d "%SOURCE_DIR%"
call cargo build --release
if %errorLevel% NEQ 0 (
    echo ERRO: Falha na compilacao!
    pause
    exit /b 1
)
echo Compilacao concluida!

echo.
echo [3/8] Copiando arquivos...
echo --------------------------
copy "target\release\fc-data-api.exe" "%INSTALL_DIR%\" /Y
copy ".env" "%INSTALL_DIR%\" /Y
copy "README.md" "%INSTALL_DIR%\" /Y
copy "DEPLOYMENT_WINDOWS.md" "%INSTALL_DIR%\" /Y
echo Arquivos copiados!

echo.
echo [4/8] Parando servico antigo (se existir)...
echo --------------------------------------------
nssm stop %SERVICE_NAME% >nul 2>&1
nssm remove %SERVICE_NAME% confirm >nul 2>&1
echo Servico antigo removido!

echo.
echo [5/8] Instalando novo servico...
echo --------------------------------
nssm install %SERVICE_NAME% "%INSTALL_DIR%\fc-data-api.exe"

REM Configurações básicas
nssm set %SERVICE_NAME% AppDirectory "%INSTALL_DIR%"
nssm set %SERVICE_NAME% DisplayName "FC Data API Service"
nssm set %SERVICE_NAME% Description "API REST para consulta de dados FC (PostgreSQL)"
nssm set %SERVICE_NAME% Start SERVICE_AUTO_START

REM Configurar recuperação de falhas
nssm set %SERVICE_NAME% AppThrottle 1500
nssm set %SERVICE_NAME% AppExit Default Restart
nssm set %SERVICE_NAME% AppRestartDelay 5000

REM Configurar logs com rotação
nssm set %SERVICE_NAME% AppStdout "%INSTALL_DIR%\logs\service.log"
nssm set %SERVICE_NAME% AppStderr "%INSTALL_DIR%\logs\error.log"
nssm set %SERVICE_NAME% AppRotateFiles 1
nssm set %SERVICE_NAME% AppRotateOnline 1
nssm set %SERVICE_NAME% AppRotateBytes 10485760

REM Prioridade do processo
nssm set %SERVICE_NAME% AppPriority ABOVE_NORMAL_PRIORITY_CLASS

echo Servico instalado!

echo.
echo [6/8] Configurando recuperacao no Windows...
echo --------------------------------------------
sc.exe failure %SERVICE_NAME% reset= 86400 actions= restart/60000/restart/60000/restart/60000
echo Recuperacao configurada!

echo.
echo [7/8] Definindo permissoes...
echo -----------------------------
icacls "%INSTALL_DIR%" /grant "NT SERVICE\%SERVICE_NAME%:(OI)(CI)F" /T >nul
echo Permissoes definidas!

echo.
echo [8/8] Iniciando servico...
echo -------------------------
nssm start %SERVICE_NAME%

REM Aguardar um pouco
timeout /t 3 /nobreak >nul

REM Verificar status
echo.
echo Verificando status do servico...
echo --------------------------------
sc query %SERVICE_NAME% | findstr "RUNNING" >nul
if %errorLevel% EQU 0 (
    echo.
    echo =======================================
    echo SUCESSO! Servico instalado e rodando!
    echo =======================================
    echo.
    echo Detalhes:
    echo - Nome do servico: %SERVICE_NAME%
    echo - Diretorio: %INSTALL_DIR%
    echo - Logs em: %INSTALL_DIR%\logs\
    echo - URL: http://localhost:8080/services/api1
    echo.
    echo Proximos passos:
    echo 1. Configure o Apache proxy reverso
    echo 2. Teste com test_endpoints.bat
    echo 3. Configure firewall se necessario
    echo.
) else (
    echo.
    echo ==========================================
    echo ATENCAO: Servico instalado mas nao iniciou
    echo ==========================================
    echo.
    echo Verifique os logs em:
    echo %INSTALL_DIR%\logs\error.log
    echo.
    echo Tente iniciar manualmente:
    echo nssm start %SERVICE_NAME%
    echo.
)

echo Para desinstalar use: uninstall_service.bat
echo.
pause
