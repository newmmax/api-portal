# FC Data API - Health Check Script
# Verifica a saúde da API e reinicia o serviço se necessário

$serviceName = "FCDataAPI"
$url = "http://localhost:8080/services/api1/health"
$logFile = "C:\fcdata-api\logs\health_check.log"
$maxLogSize = 5MB

# Função para escrever log
function Write-Log {
    param($Message)
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    "$timestamp - $Message" | Out-File $logFile -Append
    
    # Rotacionar log se muito grande
    if ((Get-Item $logFile -ErrorAction SilentlyContinue).Length -gt $maxLogSize) {
        $backupFile = $logFile -replace '\.log$', "_$(Get-Date -Format 'yyyyMMdd_HHmmss').log"
        Move-Item $logFile $backupFile -Force
        Write-Log "Log rotacionado para: $backupFile"
    }
}

# Função para verificar serviço
function Test-ServiceHealth {
    try {
        $service = Get-Service -Name $serviceName -ErrorAction Stop
        return $service.Status -eq 'Running'
    } catch {
        Write-Log "ERRO: Servico $serviceName nao encontrado"
        return $false
    }
}

# Função para verificar API
function Test-ApiHealth {
    try {
        $response = Invoke-WebRequest -Uri $url -TimeoutSec 10 -UseBasicParsing -ErrorAction Stop
        
        if ($response.StatusCode -eq 200) {
            $json = $response.Content | ConvertFrom-Json
            if ($json.status -eq "healthy") {
                return $true
            } else {
                Write-Log "AVISO: API respondeu mas status nao e 'healthy': $($json.status)"
                return $false
            }
        } else {
            Write-Log "ERRO: API respondeu com status HTTP: $($response.StatusCode)"
            return $false
        }
    } catch {
        Write-Log "ERRO: Falha ao conectar na API: $_"
        return $false
    }
}

# Função para reiniciar serviço
function Restart-ApiService {
    Write-Log "Tentando reiniciar servico $serviceName..."
    
    try {
        # Parar serviço
        Stop-Service -Name $serviceName -Force -ErrorAction SilentlyContinue
        Start-Sleep -Seconds 5
        
        # Iniciar serviço
        Start-Service -Name $serviceName -ErrorAction Stop
        Start-Sleep -Seconds 10
        
        # Verificar se iniciou
        if (Test-ServiceHealth) {
            Write-Log "Servico reiniciado com sucesso"
            return $true
        } else {
            Write-Log "ERRO: Servico nao iniciou apos reinicializacao"
            return $false
        }
    } catch {
        Write-Log "ERRO: Falha ao reiniciar servico: $_"
        return $false
    }
}

# Função principal
function Start-HealthCheck {
    $healthyService = Test-ServiceHealth
    $healthyApi = $false
    
    if ($healthyService) {
        $healthyApi = Test-ApiHealth
    }
    
    if ($healthyService -and $healthyApi) {
        # Tudo OK - log apenas uma vez por hora
        $lastOkFile = "C:\fcdata-api\logs\last_ok_check.txt"
        $logOk = $true
        
        if (Test-Path $lastOkFile) {
            $lastOk = Get-Content $lastOkFile | Get-Date
            if ((Get-Date) - $lastOk -lt [TimeSpan]::FromHours(1)) {
                $logOk = $false
            }
        }
        
        if ($logOk) {
            Write-Log "Health check OK - Servico e API funcionando normalmente"
            Get-Date -Format "yyyy-MM-dd HH:mm:ss" | Out-File $lastOkFile
        }
    } else {
        # Problema detectado
        Write-Log "PROBLEMA DETECTADO - Servico: $healthyService, API: $healthyApi"
        
        # Tentar reiniciar
        $restartSuccess = Restart-ApiService
        
        if ($restartSuccess) {
            Start-Sleep -Seconds 10
            # Verificar novamente
            if (Test-ApiHealth) {
                Write-Log "Recuperacao bem-sucedida - API respondendo apos reinicializacao"
                
                # Enviar notificação (opcional)
                # Send-Notification "FC Data API recuperada automaticamente"
            } else {
                Write-Log "ERRO CRITICO: API nao responde mesmo apos reinicializacao"
                
                # Enviar alerta crítico
                # Send-CriticalAlert "FC Data API fora do ar - intervencao manual necessaria"
            }
        } else {
            Write-Log "ERRO CRITICO: Falha ao reiniciar servico - intervencao manual necessaria"
        }
    }
}

# Executar health check
try {
    Start-HealthCheck
} catch {
    Write-Log "ERRO FATAL no script de health check: $_"
    exit 1
}
