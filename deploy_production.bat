@echo off
echo ===================================
echo FC Data API - Deploy para Produção
echo ===================================
echo.

REM Criar diretório de produção
echo Criando diretório de produção...
mkdir C:\fcdata-api 2>nul

REM Compilar release
echo.
echo Compilando versão release otimizada...
cd /d "C:\XAMPP\htdocs\portaldepedidos\fc-data-api"
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERRO: Falha na compilação!
    pause
    exit /b 1
)

REM Copiar arquivos
echo.
echo Copiando arquivos para produção...
copy target\release\fc-data-api.exe C:\fcdata-api\ /Y
copy .env C:\fcdata-api\ /Y
copy README.md C:\fcdata-api\ /Y

echo.
echo Arquivos copiados com sucesso!
echo.
echo Próximos passos:
echo 1. Edite C:\fcdata-api\.env conforme necessário
echo 2. Execute install_service.bat para instalar como serviço Windows
echo 3. Configure o Apache proxy reverso conforme README.md
echo.
pause
