@echo off
REM ================================================
REM DOWNLOAD NSSM HELPER
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - BAIXAR NSSM
echo ========================================
echo.
echo Este script ajuda a baixar o NSSM
echo.

echo Opcoes:
echo.
echo 1. Baixe manualmente de: https://nssm.cc/download
echo    - Escolha: nssm-2.24.zip
echo    - Extraia: nssm-2.24\win64\nssm.exe
echo    - Copie para: C:\fcdata-api\tools\nssm.exe
echo.
echo 2. Ou use PowerShell (requer internet):
echo.

powershell -Command "$PSVersionTable.PSVersion" >nul 2>&1
if %errorlevel% eq 0 (
    echo Baixar com PowerShell? (S/N)
    set /p baixar=
    if /i "%baixar%" equ "S" (
        echo.
        echo Baixando NSSM...
        powershell -Command "Invoke-WebRequest -Uri 'https://nssm.cc/release/nssm-2.24.zip' -OutFile 'C:\fcdata-api\tools\nssm.zip'"
        
        if exist "C:\fcdata-api\tools\nssm.zip" (
            echo [OK] Download concluido
            echo.
            echo Extraia manualmente:
            echo   - nssm-2.24\win64\nssm.exe
            echo   - Para: C:\fcdata-api\tools\
        ) else (
            echo [ERRO] Falha no download
        )
    )
) else (
    echo PowerShell nao disponivel. Baixe manualmente.
)

echo.
pause