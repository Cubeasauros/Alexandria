//#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] use rocket::post;
#[macro_use] use rocket_contrib;
#[macro_use] use serde;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use super::CodexDb;
use mysql::params;
use mysql;
use jsonwebtoken;
use jsonwebtoken::{decode, encode, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use super::jwt_handles;
















#[derive(Serialize, Deserialize)]
pub struct Message {
    id: Option<usize>,
}

// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/<id>", format = "json", data = "<message>")]
pub fn new(mut conn : CodexDb,id: usize, message: Json<Message>) -> JsonValue {
    conn.query(r"CREATE  TABLE user(name text,
        reg_no text,
        email text,
        ph_no text,
        password text)").unwrap();
    if id==1 {
        json!({
            "status": "ok",
            "message": "Table created"
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
pub fn register( mut conn:CodexDb,message: Json<NewUser>) -> JsonValue {

    let result=conn.prep_exec(r#"INSERT INTO user(name,reg_no,email,ph_no ,password ) VALUES(:name,:reg_no,:email,:ph_no,:password);"#,
    params!{"name"=>message.0.name,
    "reg_no"=>message.0.reg_no,
    "email"=>message.0.email,
    "ph_no"=>message.0.ph_no,
    "password"=>message.0.password}).unwrap();
    //println!("{:?}",out );



        json!({
            "status": "ok",
            "message": "user created"
        })

}




#[derive(Debug, Serialize, Deserialize)]
pub struct LoginToken {
    pub reg_no: String,
    pub exp: usize,
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
pub fn login( mut conn:CodexDb,message: Json<Login>) -> JsonValue {
        let mut out:Vec<String>=Vec::new();
        out=conn.prep_exec(r"SELECT password FROM user WHERE reg_no=:reg_no;",
        params!{"reg_no"=>&message.0.reg_no}).map(|result|{
        result.map(|x| x.unwrap()).map(|row| {
                    let (password) = mysql::from_row(row);
                    password
                }).collect()
            }).unwrap();


        //generates authtoken here
        if out.is_empty(){
            return(
                json!({
                    "status": "error",
                    "message": "user not available in the database"
                })
            )
        }



        if out[0]==message.0.password{

            let claims = LoginToken {
              reg_no: message.0.reg_no,
              exp: SystemTime::now()
                  .duration_since(UNIX_EPOCH)
                  .unwrap()
                  .as_secs() as usize
                  + 3600,
          };

          let token=Some(encode(&Header::default(), &claims, "secret".as_ref()).unwrap());
          json!({
                "status": "ok",
                "message": token})
            }else{
        json!({
                "status": "Wrong credentials",
                "message": "user not logged in"
            })
        }
    }



#[derive(Serialize,Deserialize)]
pub struct ProfileReturn{
        name:String,
        email:String,
        reg_no:String,
        room_no:String,
}

#[derive(Serialize,Deserialize)]
pub struct Profile{
    jwt_token:String,
}
//returns profile
#[post("/profile", format = "json", data = "<message>")]
pub fn profile(mut conn:CodexDb, message: Json<Profile>) -> JsonValue {
    let out=jwt_handles::jwt_decoder(message.0.jwt_token);

    match out{
        Some(value)=>{

            let mut output:Vec<ProfileReturn>=Vec::new();
            output=conn.prep_exec(r"SELECT name,email,reg_no,room_no FROM user WHERE reg_no=:reg_no;",
            params!{"reg_no"=>&value.reg_no}).map(|result|{
            result.map(|x| x.unwrap()).map(|row| {
                        let (name,email,reg_no,room_no) = mysql::from_row(row);
                        ProfileReturn{
                            name:name,
                            email:email,
                            reg_no:reg_no,
                            room_no:room_no
                        }

                    }).collect()
                }).unwrap();

            //bug possibiity

            json!({
                "status": "ok",
                "message": &output[0]
            })
        }
        None=>{
            json!({
                "status": "failed",
                "message": "Invalid jwt token"
            })
        }
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
pub fn list_all_books(mut conn:CodexDb,message:Json<Disp>) -> JsonValue {

        let mut booklist=Books{books:Vec::new()};

        booklist.books=conn.prep_exec(r"SELECT isbn_no,image,title FROM books;",
        ()).map(|result|{
            result.map(|x| x.unwrap()).map(|row| {
            // ⚠️ Note that from_row will panic if you don't follow your schema
            let (isbn_no,image,name) = mysql::from_row(row);
            Book{
                isbn_no:isbn_no,
                image:image,
                name:name,
            }
        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    }).unwrap(); // Unwrap `Vec<Payment>`

        json!({
            "books":booklist.books

        })
    }









//Request for book
#[derive(Serialize,Deserialize)]
pub struct RequestBook{
    book_no:String,
    auth_token:String,
}

//buy book
#[post("/buy", format = "json", data = "<message>")]
pub fn request_book(mut conn:CodexDb, message: Json<RequestBook>) -> JsonValue {

    conn.prep_exec(r#"UPDATE books SET status="sold" WHERE book_no=:book_no"#,
    params!{"book_no"=>&message.0.book_no}).unwrap();


    //change status of book here


        json!({
            "status": message.0.book_no,
            "message": "book status changed to pending "
        })

}












//delete book
#[derive(Serialize,Deserialize)]
pub struct DeleteBook{
    book_no:String,
    user_auth:String
}

//deletes  book
#[post("/delete", format = "json", data = "<message>")]
pub fn delete_book( mut conn:CodexDb,message: Json<DeleteBook>) -> JsonValue {
    //if user auth == auth for user
        conn.prep_exec(r"DELETE FROM books WHERE book_no=:book_no;",
        params!{"book_no"=>&message.0.book_no}).unwrap();

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









//Publish new book
#[derive(Serialize,Deserialize)]
pub struct NewBook{
    image:String,
    title:String,
    isbn_no:String,
    description:String,
    reg_no:String ,
    book_no:String,
    price:String
}

//publish
#[post("/new_book", format = "json", data = "<message>")]
pub fn new_book(mut conn:CodexDb, message: Json<NewBook>) -> JsonValue {


    conn.prep_exec(r"INSERT INTO books(image ,title ,isbn_no ,description,reg_no,book_no,status,price) VALUES
    (:image,:title,:isbn_no,:description,:reg_no,:book_no,:status,:price);",
    params!{"image"=>message.0.image,
            "title"=>message.0.title,
            "isbn_no"=>message.0.isbn_no,
            "description"=>message.0.description,
            "reg_no"=>message.0.reg_no,
            "book_no"=>&message.0.book_no,
            "status"=>"unsold",
            "price"=>message.0.price}).unwrap();
            //unsold or requester


    //upload new  book here
    //handle error case
    //create code for generating book id
    //get reg_no from user

    if true {
        json!({
            "status": "book created ",
            "message": message.0.book_no
        })
    } else {
        json!({ "status": "error" })
    }
}
