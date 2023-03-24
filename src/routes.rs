mod models;
mod db;
mod repository;

use rocket::{serde::json::Json, Request};
use models::{
    User, 
    CreateUser, 
    CreatedUser, 
    Product, 
    OrderHeader,
    OrderItem, 
    OrderTransportadora, 
    OrderClient, 
    Cliente, 
    Tabelas, 
    Url, 
    OrderResponse};

use db::{get_sql, get_json, get_by_id, get_many_by_id};
use repository::create_user;

use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

const WITH_WHERE: &str = " WHERE ";
const WITH_LIMIT: &str = " LIMIT ";
const WITH_OFFSET: &str = " OFFSET ";
const DEF_PAGINATION: u16 = 50;

#[get("/")]
pub async fn api() -> Redirect {
    Redirect::to(uri!("/", welcome(user = "visitante")))
}

#[get("/api/<user>")]
pub async fn welcome(user: &str) -> Template {
    let url_list = get_sql::<Url>("SELECT url, verb, sample FROM api ORDER BY url ").await;
    
    Template::render("index", context! {
        title: "Bem vindo",
        items: url_list,
        user: Some(user),
    })
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("error/404", context! {
        uri: req.uri()
    })
}

#[get("/list")]
pub async fn listar_tabelas() -> Json<Vec<Tabelas>> { 
    get_json::<Tabelas>("SELECT name FROM sqlite_schema ORDER BY name desc ").await
}

#[get("/user")]
pub async fn get_all() -> Json<Vec<User>> { 
    get_json::<User>("select * from usuarios").await
}

#[get("/user/<id>")]
pub async fn get_one(id: i64) -> Json<User> {  
    Json(get_by_id::<User>("SELECT * FROM usuarios where id = ?", id).await)
}

#[post("/user/create", format="json", data="<data>")]
pub async fn register(data: Json<CreateUser>) -> Json<CreatedUser> { 
    
    let user = CreateUser
        { nome: data.nome.clone(),
          login: data.login.clone(),
          senha: data.senha.clone(),
          email: data.email.clone()};

   let created_user = create_user(user)
    .await;

    Json(created_user)
}

#[get("/order/<id>")]
pub async fn get_order(id: i64) -> Json<OrderResponse> {  
    // get_json::<User>(r#"insert into usuarios (nome, login, email, senha) values ("Administrador", "admin", "admin@", "123") returning id, username, email"#)
    // .await
    
    const SELECT_ONE_PED: &str = include_str!("../sql/ped/ped.sql");
    const SELECT_PED_CLI: &str = include_str!("../sql/ped/ped_cli.sql");    
    const SELECT_PED_TRA: &str = include_str!("../sql/ped/ped_tra.sql");    
    const SELECT_PED_ITENS: &str = include_str!("../sql/ped/ped_itens.sql");

    let header = get_by_id::<OrderHeader>(SELECT_ONE_PED, id).await;
    let client = get_by_id::<OrderClient>(SELECT_PED_CLI, id).await;
    let transportadora = get_by_id::<OrderTransportadora>(SELECT_PED_TRA, id).await;
    let itens =  get_many_by_id::<OrderItem>(SELECT_PED_ITENS, id).await;

    let order_response = OrderResponse {
        header,
        client,
        transportadora,
        itens
    };
    Json(order_response)
}

// Produtos
#[get("/prod")]
pub async fn get_prod() -> Json<Vec<Product>> { 
    const SELECT_ALL_PROD: &str = include_str!("../sql/all_prod.sql");
    get_json::<Product>(SELECT_ALL_PROD).await
}

fn fmt_query(json: &str, query: &str, page: Option<u16>, pagination: Option<u16>) -> String {
    let mut result = String::from(json);
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

    print!("{json}");
    result
  
}

#[get("/prod?<query>&<page>&<pagination>")]
pub async fn get_prod_query(query: &str, page: Option<u16>, pagination: Option<u16> ) -> Json<Vec<Product>> {  
    const SELECT_ALL_PROD: &str = include_str!("../sql/all_prod.sql");
    get_json::<Product>(&fmt_query(SELECT_ALL_PROD, query, page, pagination)).await
}

#[get("/prod/<id>")]
pub async fn get_one_product(id: i64) -> Json<Product> {  
    const SELECT_ONE_PROD: &str = include_str!("../sql/prod.sql");
    Json(get_by_id::<Product>(SELECT_ONE_PROD, id).await)
}
    
// Order
//todo! Add itens
#[post("/oder/new")]
pub async fn new_order() -> Json<Vec<OrderHeader>> {  
    get_json::<OrderHeader>(r#"insert into pedidos (cliente_id, empregado_id, pedido_data, transportadora_id) values (?, ?, CURRENT_TIMESTAMP, ?) returning (pedido_id, cliente_id, empregado_id, pedido_data, transportadora_id, status_id)"#)
    .await
}

#[post("/oder/<id>/move")]
pub async fn update_order(id: i64) -> Json<OrderHeader> {  
    let json = r#"""
    update pedidos set (status_id = status_id + 1 ) 
    where (pedido_id = ?) 
    and status_id < (select max(status_id) from pedido_status)
    returning (pedido_id, cliente_id, empregado_id, pedido_data, transportadora_id, status_id)"#;
    Json(get_by_id::<OrderHeader>(json, id)
    .await)
}

// Clientes
#[get("/cli")]
pub async fn get_cli() -> Json<Vec<Cliente>> { 
    const SELECT_ALL_CLI: &str = include_str!("../sql/all_cli.sql");
    get_json::<Cliente>(SELECT_ALL_CLI).await
    //todo! Add pagination
}

#[get("/cli/<id>")]
pub async fn get_one_cli(id: i64) -> Json<Cliente> {  
    const SELECT_ONE_CLI: &str = include_str!("../sql/cli.sql");
    Json(get_by_id::<Cliente>(SELECT_ONE_CLI, id).await)
}

#[get("/cli?<query>&<page>&<pagination>")]
pub async fn get_cli_query(query: &str, page: Option<u16>, pagination: Option<u16> ) -> Json<Vec<Cliente>> {  
    const SELECT_ALL_CLI: &str = include_str!("../sql/all_cli.sql");
    get_json::<Cliente>(&fmt_query(SELECT_ALL_CLI, query, page, pagination)).await
}


