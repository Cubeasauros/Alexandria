use diesel::dsl::max;
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
        let result=diesel::insert_into(users::table)
        .values(&message.0)
        .execute(&conn.0);

    if let Ok(val)=result{
    json!({
            "success": true,
            "message": "user created",

        })
    }
    else{
        json!({
                "success": false,
                "message": "user not created",

            })
    }
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
                "success":true,
                "message":"User logged in",
                "token":token
                })

            }else{
                json!({
                    "success":false,
                    "message":"invalid Credentials",
                    "token":""

                })
            }
        }

        Err(out)=>{

            json!({
                "success":false,
                "message":"invalid Credentials",
                "token":""

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
    use super::schema::books::dsl::*;
    let out=jwt_handles::jwt_decoder(&message.0.token).unwrap();

    let prof_books:Vec<ProfBooks> = books.filter(owner_reg_no.eq(&out.reg_no))
                            .select((isbn_no,image,title))
                            .load(&conn.0).unwrap();

    let result:Result<UserReg,diesel::result::Error> = users
                .filter(reg_no.eq(&out.reg_no))
                .first(&conn.0);

if let Ok(results)=result
    {
        json!({
            "success":true,
            "name":results.name,
            "email":results.email,
            "reg_no":results.reg_no,
            "room_no":results.room_no,
            "books":prof_books
        })

    }
    else{
        json!({
            "success":false,
            "name":"",
            "email":"",
            "reg_no":"",
            "room_no":"",
            "books":""

        })
    }
}



/// Show after authtoken yet to be handled
/// Show all books
#[post("/book_list", format = "json", data = "<message>")]
pub fn book_list( conn:CodexPg, message:Json<Token>) -> JsonValue {
    use super::schema::books::dsl::*;
    let out:Option<jwt_handles::LoginToken>=jwt_handles::jwt_decoder(&message.0.token);

    if let Some(val)=out{
    let results:Vec<BookFetch>=books.select((isbn_no,title,owner_reg_no,book_no)).load(&conn.0).unwrap();


    json!({
            "success": true,
            "message": "booklist",
            "data":results
        })
    }
    else{
        json!({
                "success": false,
                "message": "booklist",
                "data":""
            })

    }
}





/// Shows isbn profile
#[derive(Debug, Serialize, Deserialize)]
pub struct IsbnToken {
    pub token:String,
    pub isbn_no:String
}
/// isbn profile
#[post("/isbn_profile", format = "json", data = "<message>")]
pub fn isbn_profile( conn:CodexPg, message:Json<IsbnToken>) -> JsonValue {
    use super::schema::books::dsl::*;
    let out:Option<jwt_handles::LoginToken>=jwt_handles::jwt_decoder(&message.0.token);

    if let  Some(val)=out{
    let results:Vec<BookFetch>=books
                                .filter(isbn_no.eq(message.0.isbn_no))
                                .select((isbn_no,title,owner_reg_no,book_no))
                                .load(&conn.0).unwrap();
        json!({
            "success": true,
            "message": "booklist",
            "data":results
        })
    }
    else{
        json!({
            "success":false,
            "message":"unauthorised login",
            "data":""
        })
    }
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
pub fn book_buy(conn:CodexPg, message:Json<BookBuy>) -> JsonValue {
    use super::schema::books::dsl::*;

    let out:Option<jwt_handles::LoginToken> = jwt_handles::jwt_decoder(&message.0.token);

    //Check with auth if user is in the database
    // create table to store books bought
    // reauthenticate
    // delete book from db


    if let Some(val)=out{
    let results:BookFetch = books
                .filter(book_no.eq(&message.0.book_no))
                .select((isbn_no,title,owner_reg_no,book_no))
                .first(&conn.0).unwrap();

        diesel::delete(books.filter(book_no.eq(message.0.book_no))).execute(&conn.0).unwrap();

    json!({
                "success": true,
                "message": results
        })
    }

    else{

        json!({
                "success": false,
                "message":""
            })

    }
}





/// New book Upload
#[derive(Serialize,Deserialize)]
pub struct BookUpload{
pub    image :String,
pub    title :String,
pub    isbn_no :String,
pub    description :String,
pub    price :i32,
pub    token:String
}


#[post("/new_book", format = "json", data = "<message>")]
pub fn new_book( conn:CodexPg, message:Json<BookUpload>) -> JsonValue {

    let out :Option<jwt_handles::LoginToken>= jwt_handles::jwt_decoder(&message.0.token);

    if let Some(out_val) = out{

    use super::schema::books::dsl::*;
    use super::schema::books;

    let get_book_num:i32=books.select(max(book_no)).execute(&conn.0).unwrap() as i32;

        let upload_data = BookData{
            image :message.0.image,
            title :message.0.title,
            isbn_no :message.0.isbn_no,
            description :message.0.description,
            owner_reg_no :out_val.reg_no,
            book_no :get_book_num +1,
            price :message.0.price,

        };

        let result=diesel::insert_into(books::table)
        .values(&upload_data)
        .execute(&conn.0);

if let Ok(val) = result{
    json!({
            "success": true,
            "message": "book uploaded"

        })
    }
    else{
        json!({
                "success": false,
                "message": "internal server error"

            })
        }
    }

else{


    json!({
            "success": false,
            "message": "invalid login"

        })

    }


}








/// Deletes a book
/// user has to be logged in first
#[derive(Serialize, Deserialize)]
pub struct  BookDelete {
    pub token : String,
    pub book_no : i32
}


#[post("/delete_book", format = "json", data = "<message>")]
pub fn delete_book(conn:CodexPg, message:Json<BookDelete>) -> JsonValue{
    use super::schema::books::dsl::*;

    let out = jwt_handles::jwt_decoder(&message.0.token);
    match out {
        Some(value)=>{
            //check if owner here
            diesel::delete(books.filter(book_no.eq(message.0.book_no))).execute(&conn.0).unwrap();
            json!({
                "success":true,
                "message":"book deleted"
            })
        }
        None=>{
            json!({
                "success":false,
                "message":"Invalid login details"
            })

        }
    }
}




pub fn delete_book_general(conn:CodexPg,del_book_no:i32){
    use super::schema::books::dsl::*;
    use super::schema::books;
    diesel::delete(books.filter(book_no.eq(del_book_no))).execute(&conn.0).unwrap();
}
