use rocket::serde::{json::Json};
use sqlx::{Database, Sqlite, Connection, SqliteConnection};
use dotenv_codegen::dotenv;

pub async fn get_json<T>(sql: &str) -> Json<Vec<T>> 
where
   T: for<'r> sqlx::FromRow<'r, <Sqlite as Database>::Row>,
   T: Send,
   T: Unpin,
{
    Json(get_sql::<T>(sql).await)
}

pub async fn get_sql<T>(sql: &str) -> Vec<T>
where
   T: for<'r> sqlx::FromRow<'r, <Sqlite as Database>::Row>,
   T: Send,
   T: Unpin,
{ 
    let mut connection: SqliteConnection = SqliteConnection::connect(dotenv!("DATABASE_URL"))
        .await
        .unwrap();

 sqlx::query_as::<_, T>(sql)
        .fetch_all(&mut connection)
        .await
        .unwrap()
}

pub async fn get_many_by_id<T>(sql: &str, id: i64) -> Vec<T> 
where
   T: for<'r> sqlx::FromRow<'r, <Sqlite as Database>::Row>,
   T: Send,
   T: Unpin,
{ 
    let mut connection: SqliteConnection = SqliteConnection::connect(dotenv!("DATABASE_URL"))
        .await
        .unwrap();

    sqlx::query_as::<_, T>(sql)
        .bind(id)
        .fetch_all(&mut connection)
        .await
        .unwrap()
}

pub async fn get_by_id<T>(sql: &str, id: i64) -> T 
where
   T: for<'r> sqlx::FromRow<'r, <Sqlite as Database>::Row>,
   T: Send,
   T: Unpin,
{ 
    let mut connection: SqliteConnection = SqliteConnection::connect(dotenv!("DATABASE_URL"))
        .await
        .unwrap();

    sqlx::query_as::<_, T>(sql)
        .bind(id)
        .fetch_one(&mut connection)
        .await
        .unwrap()
}