select
    transportadora_id as id,
    transportadora_nome as nome,
    fone
from transportadoras
where id = (
        select transportadora_id
        from pedidos
        where pedidos.pedido_id = ?
    )