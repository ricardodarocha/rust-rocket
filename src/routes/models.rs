use rocket::serde::{Serialize, Deserialize};
use sqlx::{FromRow};
use chrono::prelude::*;


#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Tabelas (String);

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser { 
    nome: String,
    login: String,
    senha: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User { 
    id: i64,
    nome: String,
    login: String,
    senha: String,
    email: String,
    criado: DateTime<Utc>,
 }
// #[derive(Serialize, Deserialize, Debug, FromRow)]
// pub struct Categoria {
//     categoria_id: i64,
//     categoria_nome: String,
//     descricao: String,
// }

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Product { 
    produto_id: i64,
    produto_nome: String,
    categoria_id: i64,
    categoria_nome: String,
    categoria_descricao: String,
    unidade: String,
    preco: f32,
    estoque: f32,
 }
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Order { 
    pedido_id: i64, 
    cliente_id: i64, 
    empregado_id: i64, 
    pedido_data: DateTime<Utc>, 
    transportadora_id: i64,
    status_id: i64,
 }
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Cliente { 
    cliente_id: i64, 
    cliente_nome: Option<String>, 
    contato_nome: Option<String>, 
    endereco: Option<String>,
    cidade: Option<String>, 
    cep: Option<String>, 
    pais: Option<String>,
 }