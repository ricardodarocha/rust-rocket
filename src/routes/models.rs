use rocket::serde::{Serialize, Deserialize};
use sqlx::{FromRow};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser { 
    username: String,
    password: String,
    email: String,
 }

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User { 
    id: i64,
    username: String,
    email: String,
    // created: Chrono,
 }
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Product { 
    productid: i64,
    productname: String,
    categoryid: i64,
    categoryname: String,
    description: String,
    unit: String,
    price: f64,
 }