================================
all requests are POST
================================

/signup
request
{
  "name":String,
  "reg_no":String,
   "email":String,
   "ph_no":String,
   "password":String,
   "room_no":String,
}

response
    {
          "success": true,
          "message": "user created",

      }

--------------------------------------

/login
request
{
  "reg_no":String
  "password":String
}

response
{
  "success":true
  "message":String
  "token":String
}

----------------------------
/profile
request
{
  "token":String
}

response
{
  "success":bool
  "name":String
  "email":String
  "reg_no":String
  "room_no":String
  "books":[{
            "isbn_no":String,
            "image":String",
            "title":String
            }]
}

-------------------------------------
/book_list
request
{
  "token":String
}

response
{
          "success": bool,
          "message": String,
          "data":[{
            "isbn_no":String,
            "title":String,
            "owner_reg_no":String,
            "book_no":String
            }]
}
----------------------------------

/isbn_profile

request
{
"token":String,
"isbn_no":String
}

response
{
          "success": bool,
          "message": String,
          "data":[{
            "isbn_no":String,
            "title":String,
            "owner_reg_no":String,
            "book_no":String
            }]
}

-------------------------------------
/buy
request
{
"token":String,
"isbn_no":String
}

response

  {
                "success": true,
                "message": {
                  "isbn_no":String,
                  "title":String,
                  "owner":String,
                  "reg_no":String,
                  "book_no":String
                }
  }

  //that book is deleted after this operation

------------------------------------
/new_book
request
{
      "image" :String,
      "title" :String,
      "isbn_no" :String,
      "description" :String,
      "price" :i32,
      "token":String
}

response
{
          "success": bool,
          "message": String

}


--------------------------------
/delete_book
request
{
  token : String,
  book_no : i32
}

response

{
    "success":bool,
    "message":String
  }
