mod routes;

use routes::{
    get_all, get_cli, get_one, get_one_cli, get_one_product, get_prod, new_order, register,
    update_order,
};
#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            get_one,
            get_all,
            register,
            get_prod,
            get_one_product,
            new_order,
            update_order,
            get_cli,
            get_one_cli
        ],
    )
}
