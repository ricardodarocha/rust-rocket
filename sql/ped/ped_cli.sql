select
    cliente_id as id,
    cliente_nome as nome,
    contato_nome as contato,
    endereco,
    cidade
from clientes
where id = (
        select cliente_id
        from pedidos
        where pedidos.pedido_id = ?
    )