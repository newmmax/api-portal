@echo off
REM ================================================
REM TESTE RAPIDO DA FC DATA API
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - TESTE RAPIDO
echo ========================================
echo.

REM Usar PowerShell para Windows Server 2012
echo 1. Health Check...
echo -----------------
powershell -Command "try { (New-Object Net.WebClient).DownloadString('http://localhost/services/api1/health') } catch { Write-Host 'Erro ao conectar' -ForegroundColor Red }"
echo.
echo.

REM Login com PowerShell
echo 2. Teste de Login...
echo --------------------
echo {"username":"admin_prod","password":"Pr0duc@0_FC_2025!Art3s@n@l"} > temp_test.json
powershell -Command "try { $body = Get-Content temp_test.json; $result = Invoke-RestMethod -Uri 'http://localhost/services/api1/auth/login' -Method POST -Body $body -ContentType 'application/json'; Write-Host $result } catch { Write-Host 'Erro no login' -ForegroundColor Red }"
del temp_test.json >nul 2>&1
echo.
echo.

echo ========================================
echo Se voce viu:
echo - "API rodando e banco conectado" no health
echo - Um token JWT no login
echo.
echo A API esta funcionando corretamente!
echo ========================================
echo.
pause