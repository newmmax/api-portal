-- Script SQL para testar no SQL Server Management Studio ou similar
-- Conectar em: 10.216.1.11 / Database: sys_pedidos / User: sa

-- 1. Verificar se a tabela existe
SELECT TABLE_NAME 
FROM INFORMATION_SCHEMA.TABLES 
WHERE TABLE_TYPE = 'BASE TABLE' 
AND TABLE_NAME LIKE '%cliente%'

-- 2. Se existir 'clientes', verificar estrutura
SELECT TOP 1 * FROM clientes

-- 3. Verificar quantidade de registros
SELECT COUNT(*) as total FROM clientes

-- 4. Possíveis nomes alternativos da tabela
SELECT TOP 1 * FROM cliente   -- sem 's'
SELECT TOP 1 * FROM Clientes  -- com 'C' maiúsculo
SELECT TOP 1 * FROM CLIENTES  -- tudo maiúsculo
SELECT TOP 1 * FROM tb_clientes
SELECT TOP 1 * FROM tbl_clientes