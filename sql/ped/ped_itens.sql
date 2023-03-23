select
    row_number() OVER ( ORDER BY pedido_detalhe_id ) as seq,
    Pro.produto_nome as nome,
    Pro.categoria_id as categoria,
    C.categoria_nome,
    Pro.unidade,
    PD.preco,
    PD.quantidade,
    PD.preco * PD.quantidade as total
from pedidos P
    Join pedido_detalhes PD on P.pedido_id = PD.pedido_id
    Join produtos Pro on PD.produto_id = Pro.produto_id
    Join categorias C on C.categoria_id = Pro.categoria_id
where PD.pedido_id = ?