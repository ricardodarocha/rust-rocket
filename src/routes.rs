mod models;
mod db;

use rocket::serde::json::Json;
use models::{User, Product};
use db::{get_sql, get_by_id};

#[get("/all")]
pub async fn get_all() -> Json<Vec<User>> { 
    get_sql::<User>("select * from users").await
}

#[get("/user/<id>")]
pub async fn get_one(id: i64) -> Json<User> {  
    get_by_id::<User>("SELECT * FROM users where id = ?", id).await
}

#[post("/user/new")]
pub async fn register() -> Json<Vec<User>> {  
    get_sql::<User>(r#"insert into users (username, email, password) values ("admin", "admin@", "123") returning id, username, email"#)
    .await
}

// Produtos
#[get("/prod/all")]
pub async fn get_prod() -> Json<Vec<Product>> { 
    get_sql::<Product>("select * from view_products").await
}

#[get("/prod/<id>")]
pub async fn get_one_product(id: i64) -> Json<Product> {  
    get_by_id::<Product>("SELECT * FROM View_Products where id = ?", id).await
}
