//#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] use rocket::post;
#[macro_use] use rocket_contrib;
#[macro_use] use serde;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};



#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<usize>,
    contents: String
}


// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/<id>", format = "json", data = "<message>")]
fn new(id: usize, message: Json<Message>) -> JsonValue {

    if id==1 {
        json!({
            "status": "error",
            "reason": "ID exists. Try put."
        })
    } else {
        json!({ "status": "ok" })
    }
}
