use super::super::diesel;
use super::models::*;
use super::schema::*;
use super::super::CodexPg;
use rocket_contrib::json::{Json, JsonValue};
use diesel::prelude::*;
use super::super::jwt_handles;


/// New user Registration
#[post("/signup", format = "json", data = "<message>")]
pub fn signup( conn:CodexPg, message:Json<UserReg>) -> JsonValue {

        use super::schema::users;
        diesel::insert_into(users::table)
        .values(&message.0)
        .execute(&conn.0).
        unwrap();

    json!({
            "success": true,
            "message": "user created",
            "id":message.0
        })
}









/// User Login
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginToken {
    pub reg_no: String,
    pub exp: usize,
}
#[derive(Serialize,Deserialize)]
pub struct Login{
    reg_no:String,
    password:String,
}

#[post("/login", format = "json", data = "<message>")] // change method
pub fn login( conn:CodexPg, message:Json<Login>) -> JsonValue{
    use super::schema::users::dsl::*;
    let results:Result<UserReg,diesel::result::Error>=users
                .filter(reg_no.eq(&message.0.reg_no))
                .first(&conn.0);

    match results{
        Ok(out)=>
        {
            if out.password==message.password{
            let token=jwt_handles::jwt_encoder(message.0.reg_no);
            json!({
                "result":"ok",
                "message":"User loggend in",
                "token":token
                })

            }else{
                json!({
                    "result":"failed",
                    "message":"invalid Credentials"

                })
            }
        }

        Err(out)=>{

            json!({
                "result":"failed",
                "message":"invalid Reg_no"

                })
            }
        }
}




/// Struct to contain auth_token
/// Wrong authToken is yet to be handled
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token:String
}
/// Show Profile
#[post("/profile", format = "json", data = "<message>")]
pub fn profile( conn:CodexPg, message:Json<Token>) -> JsonValue {
    use super::schema::users::dsl::*;
    let out=jwt_handles::jwt_decoder(&message.0.token).unwrap();
    let results:UserReg=users
                .filter(reg_no.eq(&out.reg_no))
                .first(&conn.0).unwrap();


    json!({
            "success": true,
            "message": "user created",
            "name":results.name,
            "reg_no":results.reg_no,
            "email":results.email,
            "ph_no":results.ph_no
        })
}





/// Show after authtoken yet to be handled
/// Show all books
#[post("/book_list", format = "json", data = "<message>")]
pub fn book_list( conn:CodexPg, message:Json<Token>) -> JsonValue {
    use super::schema::books::dsl::*;
    let out=jwt_handles::jwt_decoder(&message.0.token).unwrap();

    let results:Vec<BookFetch>=books.select((isbn_no,title,owner_reg_no)).load(&conn.0).unwrap();


    json!({
            "success": true,
            "message": "booklist",
            "data":results
        })
}





/// Buy book
#[derive(Serialize,Deserialize)]
pub struct BookBuy{
    pub token:String,
    pub book_no:i32
}
/// Show after authtoken yet to be handled
/// Buy books
#[post("/buy", format = "json", data = "<message>")]
pub fn book_buy( conn:CodexPg, message:Json<BookBuy>) -> JsonValue {
    use super::schema::books::dsl;
    let book_no=&message.0.book_no;
    let out=jwt_handles::jwt_decoder(&message.0.token).unwrap();

    //let results:Vec<BookFetch>=books.select((isbn_no,title,owner_reg_no)).load(&conn.0).unwrap();


    json!({
            "success": true,
            "message": "booklist"
        })
}





/// New book Registration
#[derive(Serialize,Deserialize)]
pub struct BookUpload{
pub    image :String,
pub    title :String,
pub    isbn_no :String,
pub    description :String,
pub    book_no :i32,
pub    price :i32,
pub    token:String
}


#[post("/new_book", format = "json", data = "<message>")]
pub fn new_book( conn:CodexPg, message:Json<BookUpload>) -> JsonValue {




        let upload_data= BookData{
            image :message.0.image,
            title :message.0.title,
            isbn_no :message.0.isbn_no,
            description :message.0.description,
            owner_reg_no :"not_yet".to_string(),
            book_no :message.0.book_no,
            price :message.0.price,

        };



        use super::schema::books;
        diesel::insert_into(books::table)
        .values(&upload_data)
        .execute(&conn.0).
        unwrap();

    json!({
            "success": true,
            "message": "book uploaded"

        })
}
