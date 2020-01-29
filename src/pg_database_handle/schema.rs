// user data
table!{
    users(reg_no){
        name -> Text,
        reg_no ->Text,
        email -> Text,
        ph_no -> Text,
        password -> Text,
        room_no -> Text,
    }
}


//book data
table!{
    books(book_no){
        image -> Text,
        title ->Text,
        isbn_no -> Text,
        description -> Text,
        owner_reg_no -> Text,
        book_no -> Integer,
        price -> Integer,
    }
}
