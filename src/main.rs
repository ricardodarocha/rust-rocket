#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};
use sqlx::{FromRow, Connection};

//todo! Move to models.rs
#[derive(Serialize, Deserialize, Debug, FromRow)]
struct User { 
    id: i64,
    name: String,
    age: i64,
 }

use sqlx::{Database, Sqlite, SqliteConnection};
//todo! Move to routes.rs

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        get_one,
        get_all,
        ])
}

#[get("/all")]
async fn get_all() -> Json<Vec<User>> { 
    get_sql::<User>("select * from users").await
}


#[get("/one/<id>")]
async fn get_one(id: u32) -> Json<User> {  
    get_by_id::<User>("SELECT * FROM users where id = ?", id).await
}

//todo! Move to database.rs
#[macro_use]
extern crate dotenv_codegen;

async fn get_sql<T>(sql: &str) -> Json<Vec<T>> 
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

    return Json(result);
}

async fn get_by_id<T>(sql: &str, id: u32) -> Json<T> 
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

    return Json(result);
}