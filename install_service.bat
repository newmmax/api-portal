@echo off
echo ===================================
echo FC Data API - Instalação como Serviço
echo ===================================
echo.

REM Verificar se está rodando como admin
net session >nul 2>&1
if %errorLevel% NEQ 0 (
    echo ERRO: Este script precisa ser executado como Administrador!
    echo.
    pause
    exit /b 1
)

REM Verificar se NSSM está instalado
where nssm >nul 2>&1
if %errorLevel% NEQ 0 (
    echo ERRO: NSSM não encontrado!
    echo.
    echo Baixe o NSSM em: https://nssm.cc/download
    echo Extraia e adicione ao PATH do sistema.
    echo.
    pause
    exit /b 1
)

REM Remover serviço antigo se existir
echo Removendo serviço antigo se existir...
nssm stop FCDataAPI >nul 2>&1
nssm remove FCDataAPI confirm >nul 2>&1

REM Instalar novo serviço
echo.
echo Instalando serviço FCDataAPI...
nssm install FCDataAPI "C:\fcdata-api\fc-data-api.exe"

REM Configurar serviço
nssm set FCDataAPI AppDirectory "C:\fcdata-api"
nssm set FCDataAPI DisplayName "FC Data API Service"
nssm set FCDataAPI Description "API REST para consulta de dados FC (PostgreSQL)"
nssm set FCDataAPI Start SERVICE_AUTO_START

REM Configurar logs
nssm set FCDataAPI AppStdout "C:\fcdata-api\logs\service.log"
nssm set FCDataAPI AppStderr "C:\fcdata-api\logs\error.log"
nssm set FCDataAPI AppRotateFiles 1
nssm set FCDataAPI AppRotateOnline 1
nssm set FCDataAPI AppRotateBytes 10485760

REM Criar diretório de logs
mkdir "C:\fcdata-api\logs" 2>nul

REM Iniciar serviço
echo.
echo Iniciando serviço FCDataAPI...
nssm start FCDataAPI

REM Verificar status
echo.
echo Verificando status do serviço...
sc query FCDataAPI

echo.
echo Instalação concluída!
echo.
echo API disponível em: http://localhost:8080/services/api1
echo Logs em: C:\fcdata-api\logs\
echo.
echo Para desinstalar: nssm remove FCDataAPI confirm
echo.
pause
