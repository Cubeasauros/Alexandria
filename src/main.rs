#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde;
#[macro_use] extern crate diesel;
pub mod pg_database_handle;
pub mod jwt_handles;
use rocket_contrib::json::{JsonValue};

#[database("codex_pg")]
pub struct CodexPg(diesel::PgConnection);


#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}


fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/message", routes![
            pg_database_handle::handle::signup,
            pg_database_handle::handle::login,
            pg_database_handle::handle::book_list,
            pg_database_handle::handle::profile,
            pg_database_handle::handle::new_book,

        ])
        .attach(CodexPg::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
