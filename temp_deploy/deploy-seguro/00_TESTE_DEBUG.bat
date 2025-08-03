@echo off
echo ============================================
echo   TESTE SIMPLES - DEBUG
echo ============================================
echo.
echo Este script testa se o problema eh de permissao
echo.

REM Teste 1: Ver diretorio atual
echo Diretorio atual:
cd
echo.

REM Teste 2: Listar arquivos
echo Arquivos na pasta:
dir /B
echo.

REM Teste 3: Verificar admin
echo Verificando se eh admin:
net session >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Voce EH administrador
) else (
    echo [ERRO] Voce NAO eh administrador
    echo Execute com botao direito - Executar como administrador
)
echo.

REM Teste 4: Ver se executavel existe
echo Procurando executavel:
if exist "..\target\release\fc-data-api.exe" (
    echo [OK] Executavel encontrado
    dir "..\target\release\fc-data-api.exe"
) else (
    echo [ERRO] Executavel NAO encontrado
    echo Caminho esperado: ..\target\release\fc-data-api.exe
)
echo.

echo Pressione qualquer tecla para sair...
pause >nul
