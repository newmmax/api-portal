@echo off
REM ================================================
REM PASSO 2: CRIAR ESTRUTURA DE DIRETORIOS
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - CRIAR ESTRUTURA
echo ========================================
echo.

REM Criar diretorio principal
echo Criando diretorios...
echo.

if not exist "C:\fcdata-api" (
    mkdir C:\fcdata-api
    if %errorlevel% eq 0 (
        echo [OK] Criado: C:\fcdata-api
    ) else (
        echo [ERRO] Falha ao criar C:\fcdata-api
        pause
        exit /b 1
    )
) else (
    echo [INFO] C:\fcdata-api ja existe
)

if not exist "C:\fcdata-api\app" (
    mkdir C:\fcdata-api\app
    echo [OK] Criado: C:\fcdata-api\app
) else (
    echo [INFO] C:\fcdata-api\app ja existe
)

if not exist "C:\fcdata-api\backup" (
    mkdir C:\fcdata-api\backup
    echo [OK] Criado: C:\fcdata-api\backup
) else (
    echo [INFO] C:\fcdata-api\backup ja existe
)

if not exist "C:\fcdata-api\logs" (
    mkdir C:\fcdata-api\logs
    echo [OK] Criado: C:\fcdata-api\logs
) else (
    echo [INFO] C:\fcdata-api\logs ja existe
)

if not exist "C:\fcdata-api\tools" (
    mkdir C:\fcdata-api\tools
    echo [OK] Criado: C:\fcdata-api\tools
) else (
    echo [INFO] C:\fcdata-api\tools ja existe
)

echo.
echo Estrutura de diretorios:
echo.
echo C:\fcdata-api\
echo    - app\       (aplicacao)
echo    - backup\    (backups)
echo    - logs\      (arquivos de log)
echo    - tools\     (ferramentas)
echo.

color 0A
echo ========================================
echo    ESTRUTURA CRIADA COM SUCESSO!
echo ========================================
echo.
echo Proximo passo: Execute 03_COPIAR_ARQUIVOS.bat
echo.
pause