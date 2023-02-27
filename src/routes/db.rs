use rocket::serde::{json::Json};
use sqlx::{Database, Sqlite, Connection, SqliteConnection};
use dotenv_codegen::dotenv;

pub async fn get_sql<T>(sql: &str) -> Json<Vec<T>> 
where
   T: for<'r> sqlx::FromRow<'r, <Sqlite as Database>::Row>,
   T: Send,
   T: Unpin,
{ 
    let mut connection: SqliteConnection = SqliteConnection::connect(dotenv!("DATABASE_URL"))
        .await
        .unwrap();

 let result = sqlx::query_as::<_, T>(sql)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    Json(result)
}

pub async fn get_by_id<T>(sql: &str, id: i64) -> Json<T> 
where
   T: for<'r> sqlx::FromRow<'r, <Sqlite as Database>::Row>,
   T: Send,
   T: Unpin,
{ 
    let mut connection: SqliteConnection = SqliteConnection::connect(dotenv!("DATABASE_URL"))
        .await
        .unwrap();

 let result = sqlx::query_as::<_, T>(sql)
        .bind(id)
        .fetch_one(&mut connection)
        .await
        .unwrap();

    Json(result)
}