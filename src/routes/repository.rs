use sqlx::{Connection, SqliteConnection};

use super::models::{CreateUser, CreatedUser};
use dotenv_codegen::dotenv;



pub async fn create_user(data: CreateUser) -> CreatedUser { 

    let mut connection: SqliteConnection = SqliteConnection::connect(dotenv!("DATABASE_URL"))
        .await
        .unwrap();

    let created_user = sqlx::query_as!(
        CreatedUser,
        r#"
        insert into
            usuarios (nome, login, senha, email)
        values (
                $1,
                $2,
                $3,
                $4
            ) 
        returning     
            id,
            nome,
            login,
            email,
            criado
        "#,
        data.nome,
        data.login,
        data.senha,
        data.email        
        )
        .fetch_one(& mut connection)
        .await
        .unwrap();

    created_user
}