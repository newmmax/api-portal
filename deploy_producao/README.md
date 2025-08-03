# 🚀 DEPLOY FC DATA API - PASSO A PASSO

## 📋 CHECKLIST PRÉ-DEPLOY

Antes de começar, certifique-se de ter:
- [ ] Acesso de Administrador no servidor
- [ ] Backup do Portal de Pedidos funcionando
- [ ] Horário de menor movimento (ideal: noite)
- [ ] TeamViewer/AnyDesk como plano B

## 📁 ARQUIVOS NECESSÁRIOS

Esta pasta contém:
- `fc-data-api.exe` - Executável da API
- `.env` - Configurações de produção
- Scripts numerados de 01 a 10 para deploy seguro
- `ROLLBACK_EMERGENCIA.bat` - Para reverter tudo se necessário

## 🔧 PROCESSO DE INSTALAÇÃO

### ⚠️ IMPORTANTE: Execute TODOS como Administrador!

1. **01_VALIDACAO_PRE_DEPLOY.bat**
   - Verifica pré-requisitos
   - Checa porta 8089
   - Valida arquivos

2. **02_CRIAR_ESTRUTURA.bat**
   - Cria diretórios em C:\fcdata-api
   - Estrutura de pastas para logs e backup

3. **03_COPIAR_ARQUIVOS.bat**
   - Copia executável e configurações
   - Faz backup se já existir versão anterior

4. **04_TESTE_MANUAL.bat**
   - Executa API manualmente para teste
   - Use Ctrl+C para parar após verificar

5. **05_INSTALAR_SERVICO.bat**
   - Instala como serviço Windows
   - PRECISA do NSSM baixado antes!

6. **06_CONFIGURAR_FIREWALL.bat**
   - Configura firewall para acesso LOCAL apenas
   - Porta 8089 só aceita conexões de 127.0.0.1

7. **07_INICIAR_SERVICO.bat**
   - Inicia o serviço FCDataAPI
   - Verifica se está rodando

8. **08_CONFIGURAR_APACHE.bat**
   - Instruções para configurar proxy reverso
   - REQUER edição manual do httpd.conf

9. **09_TESTE_COMPLETO.bat**
   - Testa todos os componentes
   - Valida integração completa

10. **10_MONITORAMENTO.bat**
    - Configura monitoramento automático
    - Cria tarefa agendada

## 🔧 BAIXAR NSSM

Antes do passo 5, baixe o NSSM:
1. Acesse: https://nssm.cc/download
2. Baixe a versão para Windows
3. Extraia `nssm.exe` para `C:\fcdata-api\tools\`

## ⚙️ CONFIGURAÇÃO DO APACHE

No passo 8, você precisará editar manualmente o Apache:

1. Abra `C:\xampp\apache\conf\httpd.conf`

2. Descomente (remova #):
```apache
LoadModule proxy_module modules/mod_proxy.so
LoadModule proxy_http_module modules/mod_proxy_http.so
LoadModule headers_module modules/mod_headers.so
```

3. No final do arquivo, adicione:
```apache
# FC Data API Proxy
<Location /services/api1>
    ProxyPreserveHost On
    ProxyPass http://127.0.0.1:8089/services/api1
    ProxyPassReverse http://127.0.0.1:8089/services/api1
    ProxyTimeout 300
    RequestHeader set X-Forwarded-Proto "https"
    RequestHeader set X-Forwarded-For "%{REMOTE_ADDR}s"
</Location>
```

4. Teste: `C:\xampp\apache\bin\httpd -t`
5. Reinicie: `C:\xampp\apache\bin\httpd -k restart`

## 🚨 SE ALGO DER ERRADO

Use `ROLLBACK_EMERGENCIA.bat` para reverter tudo!

## 📊 APÓS O DEPLOY

### URLs:
- Local: http://127.0.0.1:8089/services/api1
- Via Apache: http://localhost/services/api1
- Produção: https://conexao.artesanalfarmacia.com.br/services/api1

### Credenciais Iniciais:
- Username: `admin_prod`
- Password: `Pr0duc@0_FC_2025!Art3s@n@l`

**⚠️ MUDE A SENHA APÓS O PRIMEIRO LOGIN!**

### Comandos Úteis:
```batch
# Status do serviço
C:\fcdata-api\tools\nssm.exe status FCDataAPI

# Ver logs
type C:\fcdata-api\logs\service.log

# Reiniciar
C:\fcdata-api\tools\nssm.exe restart FCDataAPI

# Parar
C:\fcdata-api\tools\nssm.exe stop FCDataAPI
```

## 📝 NOTAS IMPORTANTES

1. O Portal continua funcionando mesmo se a API falhar
2. Monitore os logs nas primeiras 24h
3. A API só aceita conexões locais (segurança)
4. Apache faz o proxy para expor externamente

---

**Boa sorte com o deploy! 🚀**