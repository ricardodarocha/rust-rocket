mod routes;

use routes::{get_one, get_all, register, get_prod, get_one_product};
#[macro_use] 
extern crate rocket;
extern crate dotenv_codegen;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        get_one,
        get_all,
        register,
        get_prod,
        get_one_product,
        ])
    }