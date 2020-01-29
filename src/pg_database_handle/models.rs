use super::schema::users;
use super::schema::books;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}



//New user registration
#[derive(Insertable,Queryable,Serialize,Deserialize)]
#[table_name="users"]
pub struct UserReg{
    pub name:String,
    pub reg_no:String,
    pub email:String,
    pub ph_no:String,
    pub password:String,
    pub room_no:String,
}

//retrieve all books
#[derive(Insertable,Queryable,Serialize,Deserialize)]
#[table_name="books"]
pub struct BookData{
    pub image :String,
    pub title :String,
    pub isbn_no :String,
    pub description :String,
    pub owner_reg_no :String,
    pub book_no :i32,
    pub price :i32,
}


#[derive(Queryable,Serialize,Deserialize)]
pub struct BookFetch{
    title :String,
    isbn_no :String,
    owner_reg_no :String,

}


#[derive(Queryable,Serialize,Deserialize)]
pub struct ProfBooks{
    isbn_no:String,
    image:String,
    title:String,

}
