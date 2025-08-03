# 🛡️ DEPLOY SEGURO - FC DATA API

## ⚠️ ANTES DE COMEÇAR - LEIA TUDO!

Este processo foi criado para ser **100% seguro** com múltiplas validações e rollback automático.

## 📋 CHECKLIST COMPLETO

### ✅ PRÉ-REQUISITOS
- [ ] Windows Server com acesso administrativo
- [ ] NSSM baixado e disponível
- [ ] Backup do servidor feito (por precaução)
- [ ] Horário de baixo movimento escolhido
- [ ] Acesso ao PostgreSQL confirmado (10.216.1.16:5432)

### 🔧 PREPARAÇÃO LOCAL (Fazer ANTES de ir ao servidor)
- [ ] Compilar release: `cargo build --release`
- [ ] Testar localmente: `test_endpoints.bat`
- [ ] Verificar que `.env` está configurado corretamente
- [ ] Copiar pasta `deploy-seguro` para o servidor

### 🚀 PROCESSO DE DEPLOY

#### 1️⃣ PRÉ-VALIDAÇÃO (5 minutos)
```batch
cd C:\caminho\do\projeto\deploy-seguro
01_PRE_VALIDACAO.bat
```
**O que faz:**
- Verifica executável compilado
- Testa conexão PostgreSQL
- Verifica porta disponível
- Confirma permissões admin
- Testa API localmente

**Se falhar:** Corrija os erros antes de continuar!

#### 2️⃣ BACKUP (2 minutos)
```batch
02_BACKUP_ATUAL.bat
```
**O que faz:**
- Salva configurações atuais
- Cria script de rollback automático
- Backup de Apache config
- Backup de serviço (se existir)

**Importante:** Anote o caminho do backup!

#### 3️⃣ DEPLOY (10-15 minutos)
```batch
03_DEPLOY_PASSO_A_PASSO.bat
```
**O que faz:**
- Para serviço antigo
- Cria estrutura de diretórios
- Copia arquivos
- Testa executável
- Instala serviço Windows
- Configura recuperação automática
- Inicia serviço
- Opção de configurar Apache

**Cada passo tem pausa para validação!**

#### 4️⃣ VALIDAÇÃO FINAL (5 minutos)
```batch
04_VALIDACAO_FINAL.bat
```
**O que faz:**
- Verifica serviço rodando
- Testa todos endpoints
- Valida logs
- Gera relatório completo

### 🔴 SE ALGO DER ERRADO

#### Rollback Imediato:
1. Vá para a pasta de backup criada
2. Execute `ROLLBACK.bat`
3. Isso irá:
   - Parar serviço novo
   - Restaurar arquivos antigos
   - Restaurar config Apache
   - Reiniciar Apache

#### Problemas Comuns:

**1. Porta 8089 em uso:**
```batch
netstat -ano | findstr :8089
taskkill /PID [numero_do_pid] /F
```

**2. Serviço não inicia:**
- Verifique `C:\fcdata-api\logs\error.log`
- Teste executável manualmente:
  ```batch
  cd C:\fcdata-api
  fc-data-api.exe
  ```

**3. "Access Denied":**
- Execute como Administrador
- Verifique permissões da pasta

**4. PostgreSQL não conecta:**
- Teste conectividade: `telnet 10.216.1.16 5432`
- Verifique firewall
- Confirme credenciais no .env

**5. Apache não funciona:**
- Verifique sintaxe: `C:\XAMPP\apache\bin\httpd.exe -t`
- Logs em: `C:\XAMPP\apache\logs\error.log`

### 📊 MONITORAMENTO PÓS-DEPLOY

#### Primeiras 24 horas:
- [ ] Verificar logs a cada 2 horas
- [ ] Monitorar uso de CPU/memória
- [ ] Testar endpoints periodicamente
- [ ] Verificar se serviço reinicia após reboot

#### Scripts úteis:
```batch
# Ver status
sc query FCDataAPI

# Ver logs
type C:\fcdata-api\logs\service.log

# Testar API
curl http://localhost:8089/services/api1/health

# Reiniciar se necessário
nssm restart FCDataAPI
```

### 🎯 CONFIGURAÇÃO APACHE (se não fez no deploy)

Adicione em `C:\XAMPP\apache\conf\extra\httpd-vhosts.conf`:

```apache
# Dentro do VirtualHost HTTPS
# Proxy para FC Data API
ProxyPass /services/api1 http://localhost:8089/services/api1
ProxyPassReverse /services/api1 http://localhost:8089/services/api1

# Headers de segurança
<Location /services/api1>
    Header always set X-Content-Type-Options "nosniff"
    Header always set X-Frame-Options "DENY"
</Location>
```

Depois reinicie Apache:
```batch
C:\XAMPP\apache\bin\httpd.exe -k restart
```

### ✅ CONFIRMAÇÃO FINAL

A API está funcionando quando:
1. `sc query FCDataAPI` mostra "RUNNING"
2. `http://localhost:8089/services/api1/health` retorna JSON
3. `https://conexao.artesanalfarmacia.com.br/services/api1/health` funciona
4. Logs não mostram erros
5. Serviço sobrevive a reinicialização

### 📞 SUPORTE

Se precisar de ajuda:
1. Colete os logs
2. Execute `04_VALIDACAO_FINAL.bat`
3. Salve o relatório gerado
4. Documente o erro específico

## 🎉 SUCESSO!

Quando tudo estiver verde:
- API rodando como serviço Windows resiliente
- Recuperação automática configurada
- Logs sendo gerados
- Acessível via HTTPS
- Pronto para produção!

---
**Tempo total estimado: 30-40 minutos**
**Risco: MÍNIMO com este processo**
