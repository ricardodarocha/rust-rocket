mod models;
mod db;

use rocket::serde::json::Json;
use models::{User, Product, Order, Cliente};
use db::{get_sql, get_by_id};

#[get("/all")]
pub async fn get_all() -> Json<Vec<User>> { 
    get_sql::<User>("select * from usuarios").await
}

#[get("/user/<id>")]
pub async fn get_one(id: i64) -> Json<User> {  
    get_by_id::<User>("SELECT * FROM usuarios where id = ?", id).await
}

#[post("/user/new")]
pub async fn register() -> Json<Vec<User>> {  
    get_sql::<User>(r#"insert into usuarios (nome, login, email, senha) values ("Administrador", "admin", "admin@", "123") returning id, username, email"#)
    .await
}

// Produtos
#[get("/prod/all")]
pub async fn get_prod() -> Json<Vec<Product>> { 
    get_sql::<Product>("select * from view_produtos").await
}

#[get("/prod/<id>")]
pub async fn get_one_product(id: i64) -> Json<Product> {  
    get_by_id::<Product>("SELECT * FROM view_produtos where id = ?", id).await
}

// Order
//todo! Add itens
#[post("/oder/new")]
pub async fn new_order() -> Json<Vec<Order>> {  
    get_sql::<Order>(r#"insert into pedidos (cliente_id, empregado_id, pedido_data, transportadora_id) values (?, ?, CURRENT_TIMESTAMP, ?) returning (pedido_id, cliente_id, empregado_id, pedido_data, transportadora_id, status_id)"#)
    .await
}

#[post("/oder/<id>/move")]
pub async fn update_order(id: i64) -> Json<Order> {  
    let sql = r#"""
    update pedidos set (status_id = status_id + 1 ) 
    where (pedido_id = ?) 
    and status_id < (select max(status_id) from pedido_status)
    returning (pedido_id, cliente_id, empregado_id, pedido_data, transportadora_id, status_id)"#;
    get_by_id::<Order>(sql, id)
    .await
}

// Clientes
#[get("/cli/all")]
pub async fn get_cli() -> Json<Vec<Cliente>> { 
    get_sql::<Cliente>("select * from view_clientes").await
    //todo! Add pagination
    // get_sql_pag::<Cliente>("select * from view_clientes LIMIT ? OFFSET ?", -1, 0).await
}

#[get("/cli/<id>")]
pub async fn get_one_cli(id: i64) -> Json<Cliente> {  
    get_by_id::<Cliente>("SELECT * FROM view_clientes where cliente_id = ?", id).await
}


