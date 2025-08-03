@echo off
REM ================================================
REM PASSO 1: VALIDACAO PRE-DEPLOY
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - VALIDACAO PRE-DEPLOY
echo ========================================
echo.
echo Windows Server 2012 R2 Detectado
echo.

echo Verificando pre-requisitos...
echo.

REM Verificar se esta rodando como Admin
net session >nul 2>&1
if %errorlevel% neq 0 (
    color 0C
    echo [ERRO] Este script deve ser executado como ADMINISTRADOR!
    echo.
    echo Clique com botao direito e selecione "Executar como administrador"
    pause
    exit /b 1
)

echo [OK] Executando como Administrador
echo.

REM Verificar arquivos necessarios
echo Verificando arquivos...
if not exist "fc-data-api.exe" (
    color 0C
    echo [ERRO] Arquivo fc-data-api.exe nao encontrado!
    echo.
    echo Diretorio atual: %CD%
    echo.
    dir /b *.exe
    echo.
    pause
    exit /b 1
)
echo [OK] fc-data-api.exe encontrado

if not exist ".env" (
    color 0C
    echo [ERRO] Arquivo .env nao encontrado!
    pause
    exit /b 1
)
echo [OK] .env encontrado
echo.

REM Verificar porta 8089
echo Verificando disponibilidade da porta 8089...
netstat -an | findstr :8089 >nul
if %errorlevel% eq 0 (
    color 0C
    echo [AVISO] Porta 8089 pode estar em uso!
    echo.
    netstat -an | findstr :8089
    echo.
    echo Deseja continuar mesmo assim? (S/N)
    set /p continuar=
    if /i "%continuar%" neq "S" (
        echo Deploy cancelado.
        pause
        exit /b 1
    )
) else (
    echo [OK] Porta 8089 esta livre
)
echo.

REM Verificar IIS (comum em Windows Server)
echo Verificando servidor web...
sc query W3SVC >nul 2>&1
if %errorlevel% eq 0 (
    echo [INFO] IIS detectado
) else (
    sc query Apache2.4 >nul 2>&1
    if %errorlevel% eq 0 (
        echo [INFO] Apache detectado
    ) else (
        echo [INFO] Nenhum servidor web detectado como servico
    )
)
echo.

color 0A
echo ========================================
echo    VALIDACAO CONCLUIDA COM SUCESSO!
echo ========================================
echo.
echo Proximo passo: Execute 02_CRIAR_ESTRUTURA.bat
echo.
pause