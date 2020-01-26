//#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] use rocket::post;
#[macro_use] use rocket_contrib;
#[macro_use] use serde;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use super::CodexDb;

use super::sq_lite_test;



#[derive(Serialize, Deserialize)]
pub struct Message {
    id: Option<usize>,
    contents: String
}






// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/<id>", format = "json", data = "<message>")]
pub fn new(mut conn : CodexDb,id: usize, message: Json<Message>) -> JsonValue {
    conn.query(r"CREATE  TABLE ment(id int, amount int ,name text)").unwrap();
    if id==1 {
        json!({
            "status": "ok",
            "reason": "ID exists. tartereTry put."
        })
    } else {
        json!({ "status": "ok" })
    }
}






//new_user
#[derive(Serialize,Deserialize,Debug)]
pub struct NewUser {
        name:String,
        reg_no:String,
        email:String,
        ph_no:String,
        password:String,
}
//Creates new user
//check errors
#[post("/register", format = "json", data = "<message>")]
pub fn register( message: Json<NewUser>) -> JsonValue {

    //database handler and checks here
    if true {
        json!({
            "status": message.0.name,
            "message": "user created"
        })
    } else {
        json!({ "status": "error" })
    }
}






//login for user
#[derive(Serialize,Deserialize)]
pub struct Login{
    reg_no:String,
    password:String,
}

//login
//create after login things
#[post("/login", format = "json", data = "<message>")]
pub fn login( message: Json<Login>) -> JsonValue {

    if true {
        json!({
            "status": message.0.reg_no,
            "message": "user logged in "
        })
    } else {
        json!({ "status": "error" })
    }
}







//retrieve book testing phase
#[derive(Serialize,Deserialize)]
pub struct Book{
    isbn_no:String,
    image:String,
    name:String,
}

#[derive(Serialize,Deserialize)]
pub struct Disp{
    name:String,
}





//list all books
#[derive(Serialize,Deserialize)]
pub struct Books{
    books:Vec<Book>,
}

//list all books
#[post("/list_all_books", format = "json", data="<message>")]
pub fn list_all_books(message:Json<Disp>) -> JsonValue {

    let book=Books{
        books:vec![Book{
        isbn_no:"hey".to_string(),
        image:"het".to_string(),
        name:"hsadgjh".to_string()
    },
    Book{
    isbn_no:"whey".to_string(),
    image:"hessdt".to_string(),
    name:"hsadgjasah".to_string()
}]
};
    if true {
        json!({
            "books":book.books

        })
    } else {
        json!({ "status": "error" })
    }
}









//Request for book
#[derive(Serialize,Deserialize)]
pub struct RequestBook{
    book_no:String,
    buyer:String,
}

//login
#[post("/request", format = "json", data = "<message>")]
pub fn request_book( message: Json<RequestBook>) -> JsonValue {


    //change statuss of book here

    if true {
        json!({
            "status": message.0.book_no,
            "message": "user logged in "
        })
    } else {
        json!({ "status": "error" })
    }
}



//deletes  book
#[post("/delete", format = "json", data = "<message>")]
pub fn delete_book( message: Json<RequestBook>) -> JsonValue {


    //delete book here
    //check if owner

    if true {
        json!({
            "status": message.0.book_no,
            "message": "book deleted"
        })
    } else {
        json!({ "status": "error" })
    }
}





//login
#[derive(Serialize,Deserialize)]
pub struct NewBook{
    image:String,
    title:String,
    isbn_no:String,
    description:String,
    unique_id:String
}

//login
#[post("/new_book", format = "json", data = "<message>")]
pub fn new_book( message: Json<NewBook>) -> JsonValue {


    //upload new  book here

    if true {
        json!({
            "status": message.0.unique_id,
            "message": "book created "
        })
    } else {
        json!({ "status": "error" })
    }
}
