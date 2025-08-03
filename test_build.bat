@echo off
echo ====================================
echo Testando API Unificada
echo ====================================
echo.

echo 1. Compilando projeto...
cargo build
if %ERRORLEVEL% NEQ 0 (
    echo ERRO: Falha na compilacao!
    pause
    exit /b 1
)

echo.
echo 2. Verificando tipos...
cargo check
if %ERRORLEVEL% NEQ 0 (
    echo ERRO: Falha na verificacao de tipos!
    pause
    exit /b 1
)

echo.
echo 3. Executando testes...
cargo test
if %ERRORLEVEL% NEQ 0 (
    echo AVISO: Alguns testes falharam
)

echo.
echo ====================================
echo Compilacao concluida com sucesso!
echo ====================================
echo.
echo Para executar a API: cargo run
echo.
pause
