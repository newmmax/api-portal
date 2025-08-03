@echo off
title FC Data API - Pre Validacao

:inicio
cls
echo ============================================
echo   FC DATA API - PRE VALIDACAO SIMPLIFICADA
echo ============================================
echo.
echo Este script NAO vai fechar sozinho!
echo.

REM Mostrar onde estamos
echo Diretorio atual:
cd
echo.

REM Ver se somos admin
echo Status de Admin:
net session >nul 2>&1 && (
    echo [OK] Voce eh administrador
) || (
    echo [AVISO] NAO eh administrador - alguns testes podem falhar
)
echo.

REM Menu simples
echo O que deseja testar?
echo.
echo 1 - Verificar executavel
echo 2 - Verificar .env
echo 3 - Testar conexao PostgreSQL
echo 4 - Verificar porta 8089
echo 5 - Executar TODOS os testes
echo 6 - Sair
echo.
choice /C 123456 /N /M "Escolha uma opcao: "

if errorlevel 6 goto fim
if errorlevel 5 goto todos
if errorlevel 4 goto porta
if errorlevel 3 goto postgres
if errorlevel 2 goto env
if errorlevel 1 goto executavel

:executavel
echo.
echo Verificando executavel...
if exist "..\target\release\fc-data-api.exe" (
    echo [OK] Encontrado!
    dir "..\target\release\fc-data-api.exe" | findstr fc-data-api
) else (
    echo [ERRO] Nao encontrado em ..\target\release\
)
echo.
pause
goto inicio

:env
echo.
echo Verificando .env...
if exist "..\.env" (
    echo [OK] Encontrado!
    echo.
    echo Porta configurada:
    type "..\.env" | findstr SERVER_PORT
) else (
    echo [ERRO] Nao encontrado!
)
echo.
pause
goto inicio

:postgres
echo.
echo Testando PostgreSQL...
ping -n 1 10.216.1.16 >nul 2>&1 && (
    echo [OK] Host 10.216.1.16 responde ao ping
) || (
    echo [AVISO] Host nao responde ao ping
)
echo.
pause
goto inicio

:porta
echo.
echo Verificando porta 8089...
netstat -an | findstr :8089 | findstr LISTENING && (
    echo [AVISO] Porta em uso!
) || (
    echo [OK] Porta livre!
)
echo.
pause
goto inicio

:todos
echo.
echo Executando todos os testes...
echo ==============================
echo.
call :executavel
call :env
call :postgres
call :porta
echo.
echo Testes concluidos!
pause
goto inicio

:fim
exit
