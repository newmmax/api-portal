@echo off
echo ============================================
echo   PREPARANDO ARQUIVOS PARA DEPLOY
echo ============================================
echo.

cd /d "C:\XAMPP\htdocs\portaldepedidos\fc-data-api"

REM Verificar se o executável existe
if not exist "target\release\fc-data-api.exe" (
    echo [AVISO] Executavel nao encontrado!
    echo.
    echo Deseja compilar agora? (S/N)
    choice /C SN /N
    if errorlevel 2 (
        echo Cancelado. Compile com: cargo build --release
        pause
        exit /b 1
    )
    
    echo.
    echo Compilando em modo release...
    echo Isso pode demorar alguns minutos...
    cargo build --release
    
    if not exist "target\release\fc-data-api.exe" (
        echo [ERRO] Falha na compilacao!
        pause
        exit /b 1
    )
)

echo.
echo [1/5] Limpando pasta temp_deploy...
if exist "temp_deploy" rmdir /S /Q "temp_deploy"
mkdir "temp_deploy"

echo.
echo [2/5] Copiando executavel...
copy "target\release\fc-data-api.exe" "temp_deploy\" >nul
if %errorlevel% == 0 (
    echo [OK] fc-data-api.exe copiado
    for %%A in ("temp_deploy\fc-data-api.exe") do echo      Tamanho: %%~zA bytes
) else (
    echo [ERRO] Falha ao copiar executavel!
    pause
    exit /b 1
)

echo.
echo [3/5] Copiando configuracao...
copy ".env" "temp_deploy\.env" >nul
if %errorlevel% == 0 (
    echo [OK] .env copiado
) else (
    echo [ERRO] Falha ao copiar .env!
    pause
    exit /b 1
)

echo.
echo [4/5] Copiando scripts de deploy...
xcopy "deploy-seguro" "temp_deploy\deploy-seguro\" /E /I /Q >nul
if %errorlevel% == 0 (
    echo [OK] Pasta deploy-seguro copiada
) else (
    echo [ERRO] Falha ao copiar deploy-seguro!
    pause
    exit /b 1
)

echo.
echo [5/5] Criando arquivos adicionais...
REM Criar .env de produção exemplo
(
echo # FC Data API - Configuracao de PRODUCAO
echo # IMPORTANTE: Ajuste antes de usar!
echo.
echo # Servidor - escutar em todas interfaces
echo SERVER_HOST=0.0.0.0
echo SERVER_PORT=8089
echo.
echo # PostgreSQL Producao
echo DATABASE_URL=postgres://rodrigo:R0drigoPgSQL@10.216.1.16:5432/fc_data
echo.
echo # JWT - MUDE A SECRET!
echo JWT_SECRET=fc_data_api_jwt_secret_artesanal_2025_PRODUCAO_MUDE_ISSO
echo JWT_EXPIRATION_HOURS=24
echo.
echo # CORS - Apenas producao
echo CORS_ALLOWED_ORIGINS=https://conexao.artesanalfarmacia.com.br
echo.
echo # API
echo API_PREFIX=/services/api1
echo.
echo # Credenciais - MUDE!
echo ADMIN_USERNAME=admin
echo ADMIN_PASSWORD=MudeEstaSenhaEmProducao2025!
echo.
echo # Logs - menos verbose em producao
echo RUST_LOG=info
) > "temp_deploy\.env.producao"

REM Criar README
(
echo # FC DATA API - ARQUIVOS DE DEPLOY
echo.
echo Esta pasta contem todos os arquivos necessarios para deploy.
echo.
echo ## Conteudo:
echo - fc-data-api.exe - Executavel da API
echo - .env - Configuracao atual (desenvolvimento)
echo - .env.producao - Exemplo de configuracao para producao
echo - deploy-seguro/ - Scripts de instalacao
echo.
echo ## IMPORTANTE:
echo.
echo 1. ANTES de executar os scripts:
echo    - Revise e ajuste o arquivo .env
echo    - Use .env.producao como base
echo    - MUDE as senhas e JWT secret!
echo.
echo 2. Execute os scripts NA ORDEM:
echo    - 01_VALIDACAO_MENU.bat (ou 01_PRE_VALIDACAO.bat)
echo    - 02_BACKUP_ATUAL.bat
echo    - 03_DEPLOY_PASSO_A_PASSO.bat
echo    - 04_VALIDACAO_FINAL.bat
echo.
echo 3. Requisitos no servidor:
echo    - Windows Server 2012+ ou Windows 10/11
echo    - Acesso administrativo
echo    - NSSM instalado
echo    - Acesso ao PostgreSQL em 10.216.1.16:5432
echo.
echo ## Inicio Rapido:
echo.
echo 1. Copie esta pasta para C:\ do servidor
echo 2. Abra CMD como Administrador
echo 3. cd C:\temp_deploy\deploy-seguro
echo 4. Execute: 01_VALIDACAO_MENU.bat
echo.
echo Boa sorte com o deploy!
) > "temp_deploy\LEIA_PRIMEIRO.txt"

echo [OK] Arquivos adicionais criados
echo.

REM Mostrar resumo
echo ============================================
echo   PREPARACAO CONCLUIDA!
echo ============================================
echo.
echo Pasta criada: temp_deploy\
echo.
echo Conteudo:
dir /B "temp_deploy"
echo.
echo Tamanho total:
dir "temp_deploy" | findstr "File(s)"
echo.
echo PROXIMO PASSO:
echo 1. Revise o arquivo temp_deploy\.env
echo 2. Copie a pasta temp_deploy para o servidor
echo 3. No servidor, execute os scripts em ordem
echo.
echo A pasta esta em:
echo %cd%\temp_deploy
echo.
pause
