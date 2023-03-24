use rocket::serde::{Serialize, Deserialize};
use sqlx::{FromRow};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Tabelas (String);

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Url {
    url: String,
    verb: String,
    sample: String,
}

#[derive(FromForm, Deserialize)]
pub struct CreateUser { 
    pub nome: String,
    pub login: String,
    pub senha: String,
    pub email: String,
}
#[derive(Serialize, FromRow)]
pub struct CreatedUser { 
    pub id: i64,
    pub nome: Option<String>,
    pub login: Option<String>,
    pub email: Option<String>,
    pub criado: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User { 
    pub id: i64,
    pub nome: String,
    pub login: String,
    pub senha: String,
    pub email: String,
    pub criado: NaiveDateTime,
 }

// #[derive(Serialize, Deserialize, Debug, FromRow)]
// pub struct Categoria {
//     categoria_id: i64,
//     categoria_nome: String,
//     descricao: String,
// }

//cadastros
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Product { 
    id: i64,
    nome: String,
    categoria: i64,
    categoria_nome: String,
    categoria_descricao: String,
    unidade: String,
    preco: f32,
    estoque: f32,
 }

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Cliente { 
    id: i64, 
    nome: Option<String>, 
    contato: Option<String>, 
    endereco: Option<String>,
    cidade: Option<String>, 
    cep: Option<String>, 
    pais: Option<String>,
 }

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct OrderHeader { 
    id: i64, 
    cliente: i64, 
    data: NaiveDate, 
    transportadora: i64,
    status: i64,
    status_nome: String,
    total: f64,
 }

 //processos
 #[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct OrderClient { 
    id: i64,
    nome: Option<String>, 
    contato: Option<String>, 
    endereco: Option<String>,
    cidade: Option<String>, 
}
 #[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct OrderTransportadora { 
    id: i64,
    nome: Option<String>, 
    fone: Option<String>, 
}
 #[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct OrderItem { 
    seq: i64,
    nome: Option<String>, 
    categoria: Option<i64>, 
    categoria_nome: Option<String>, 
    unidade: Option<String>,
    preco: Option<f64>, 
    quantidade: Option<f64>, 
    total: Option<f64>, 
}

#[derive(Serialize, Debug)]
pub struct OrderResponse {
    pub header: OrderHeader,
    pub client: OrderClient,
    pub transportadora: OrderTransportadora,
    pub itens: Vec<OrderItem>,
}