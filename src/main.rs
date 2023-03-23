mod routes;

use rocket_dyn_templates::Template;
use routes::{
    api, get_all, get_cli, get_cli_query, get_one, get_one_cli, get_one_product, get_order,
    get_prod, get_prod_query, listar_tabelas, new_order, not_found, register, update_order,
    welcome,
};
use rocket::fs::{FileServer, relative};

#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                get_one,
                get_all,
                get_order,
                register,
                get_prod,
                get_prod_query,
                get_one_product,
                new_order,
                update_order,
                get_cli,
                get_cli_query,
                get_one_cli,
                listar_tabelas,
                api,
                welcome,
            ],
        )
        .mount("/static", FileServer::from(relative!("static")))
        .register("/tera", catchers![not_found])
}
