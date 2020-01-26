#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde;
extern crate rusqlite;
extern crate base64;
use Alexandria::json_handle;
use Alexandria::CodexDb;
use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;

use rocket_contrib::json::{Json, JsonValue};




#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}






fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/message", routes![json_handle::new,json_handle::register,json_handle::login, json_handle::list_all_books])
        .attach(CodexDb::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
