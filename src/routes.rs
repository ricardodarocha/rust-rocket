mod models;
mod db;

use rocket::serde::json::Json;
use models::{User, Product, Order, Cliente, Tabelas};
use db::{get_sql, get_by_id};

const WITH_WHERE: &'static str = " WHERE ";
const WITH_LIMIT: &'static str = " LIMIT ";
const WITH_OFFSET: &'static str = " OFFSET ";
const DEF_PAGINATION: u16 = 50;

#[get("/list")]
pub async fn listar_tabelas() -> Json<Vec<Tabelas>> { 
    get_sql::<Tabelas>("SELECT name FROM sqlite_schema ORDER BY name desc ").await
}

#[get("/user")]
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
#[get("/prod")]
pub async fn get_prod() -> Json<Vec<Product>> { 
    const SELECT_ALL_PROD: &str = include_str!("../sql/all_prod.sql");
    get_sql::<Product>(SELECT_ALL_PROD).await
}

fn fmt_query(sql: &str, query: &str, page: Option<u16>, pagination: Option<u16>) -> String {
    let mut result = String::from(sql);
    if !query.is_empty() {
            result.push_str(WITH_WHERE);
            result.push_str(query);
        };

    match (pagination, page) {
        (None, Some(page_no)) => {
            result.push_str(WITH_OFFSET);
            let offset = (page_no - 1) * DEF_PAGINATION;
            result.push_str(&*offset.to_string());
        },
        (Some(limit), None) => {
            result.push_str(WITH_LIMIT);
            result.push_str(&*limit.to_string());
        },
        (Some(limit), Some(page_no)) => {
            result.push_str(WITH_LIMIT);
            result.push_str(&*limit.to_string());
            result.push_str(WITH_OFFSET);
            let offset = (page_no - 1) * limit;
            result.push_str(&*offset.to_string());
        },
        _ => (),
    };

    print!("{sql}");
    result
  
}

#[get("/prod?<query>&<page>&<pagination>")]
pub async fn get_prod_query(query: &str, page: Option<u16>, pagination: Option<u16> ) -> Json<Vec<Product>> {  
    const SELECT_ALL_PROD: &str = include_str!("../sql/all_prod.sql");
    get_sql::<Product>(&fmt_query(SELECT_ALL_PROD, query, page, pagination)).await
}

#[get("/prod/<id>")]
pub async fn get_one_product(id: i64) -> Json<Product> {  
    const SELECT_ONE_PROD: &str = include_str!("../sql/prod.sql");
    get_by_id::<Product>(SELECT_ONE_PROD, id).await
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
#[get("/cli")]
pub async fn get_cli() -> Json<Vec<Cliente>> { 
    get_sql::<Cliente>("select * from view_clientes").await
    //todo! Add pagination
    // get_sql_pag::<Cliente>("select * from view_clientes LIMIT ? OFFSET ?", -1, 0).await
}

#[get("/cli/<id>")]
pub async fn get_one_cli(id: i64) -> Json<Cliente> {  
    get_by_id::<Cliente>("SELECT * FROM view_clientes where cliente_id = ?", id).await
}


