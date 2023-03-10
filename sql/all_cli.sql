-- SQLite

SELECT
    A.cliente_id as id,
    A.cliente_nome as nome,
    A.contato_nome as contato,
    A.endereco,
    A.cidade,
    A.cep,
    A.pais
FROM Clientes A