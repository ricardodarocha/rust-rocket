-- SQLite

SELECT
    P.produto_id,
    P.produto_nome,
    C.categoria_id,
    C.categoria_nome,
    C.descricao as categoria_descricao,
    P.unidade,
    P.preco,
    P.estoque
FROM Produtos P
    inner join Categorias C on C.categoria_id = P.categoria_id 