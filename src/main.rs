#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};
use sqlx::{FromRow, Connection};

//Model todo! Move to models.rs
#[derive(Serialize, Deserialize, Debug, FromRow)]
struct User { 
    id: i64,
    name: String,
    age: i64,
 }


//database todo! Move to database.rs

use sqlx::SqliteConnection;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        get_all,
        get_by_id])
}

//routes todo! Move to routes.rs
#[macro_use]
extern crate dotenv_codegen;

#[get("/all")]
async fn get_all() -> Json<Vec<User>> { 
    let mut connection: SqliteConnection = SqliteConnection::connect(dotenv!("DATABASE_URL"))
        .await
        .unwrap();

    let result = sqlx::query_as!(User, "SELECT * FROM users")
    .fetch_all(&mut connection)
    .await
    .unwrap();

    return Json(result);
}


#[get("/one/<id>")]
async fn get_by_id(id: u32) -> Json<Vec<User>> {  
    let mut connection: SqliteConnection = SqliteConnection::connect(dotenv!("DATABASE_URL"))
        .await
        .unwrap();

    let result = sqlx::query_as!(User, "SELECT * FROM users where id = ?", id)
        .fetch_all(&mut connection)
        .await
        .unwrap();

    return Json(result);
}
