# 🚨 SOLUÇÃO: Script Fechando Sozinho

## O Problema
O script `01_PRE_VALIDACAO.bat` está fechando imediatamente porque:
1. Não está sendo executado como Administrador
2. Ou está encontrando um erro e fechando rápido demais

## 🔧 Soluções (tente na ordem):

### Solução 1: Use o arquivo auxiliar
```
1. Execute: ABRIR_01_SEM_FECHAR.bat
   (Este mantém a janela aberta)
```

### Solução 2: Execute como Administrador
```
1. Clique com botão direito em 01_PRE_VALIDACAO.bat
2. Escolha "Executar como administrador"
3. A janela deve permanecer aberta
```

### Solução 3: Use o PowerShell (mais confiável)
```
1. Clique com botão direito em 01_PRE_VALIDACAO.ps1
2. Escolha "Executar com PowerShell"
3. Se pedir, digite S para permitir execução
```

### Solução 4: Abra CMD como Admin primeiro
```
1. Aperte Windows + X
2. Escolha "Terminal (Admin)" ou "Prompt de Comando (Admin)"
3. Navegue até a pasta:
   cd C:\caminho\para\deploy-seguro
4. Execute:
   01_PRE_VALIDACAO.bat
```

### Solução 5: Debug rápido
```
1. Execute primeiro: 00_TESTE_DEBUG.bat
2. Veja o que aparece (este não fecha sozinho)
3. Me mande o resultado
```

## 📝 Se nada funcionar:

Abra um Prompt de Comando como Admin e execute:
```batch
cd C:\XAMPP\htdocs\portaldepedidos\fc-data-api\deploy-seguro
echo Teste > teste.txt
dir

REM Se funcionar, execute:
..\target\release\fc-data-api.exe --version
```

Me envie o resultado!

## ✅ Script Atualizado

O novo `01_PRE_VALIDACAO.bat`:
- Força a janela a ficar aberta
- Mostra erros claramente
- Tem mais pausas
- Cria logs detalhados
- Avisa se não for admin

**Tente novamente com a versão atualizada!**
