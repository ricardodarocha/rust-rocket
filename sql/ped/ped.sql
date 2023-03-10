SELECT
    P.pedido_id as id,
    P.cliente_id as cliente,
    P.pedido_data as data,
    P.transportadora_id as transportadora,
    P.status_id as status,
    S.status_nome as status_nome,
    coalesce(P.total, 0.0) as total
from pedidos P
    inner join status S on P.status_id = S.status_id
    where id = ?