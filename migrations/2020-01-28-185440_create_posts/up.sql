-- Your SQL goes here
CREATE TABLE users (
  name TEXT ,
  reg_no TEXT PRIMARY KEY,
  email TEXT ,
  ph_no TEXT,
  password TEXT,
  room_no TEXT
);

CREATE TABLE books(
  image TEXT,
  title TEXT,
  isbn_no TEXT,
  description TEXT,
  owner_reg_no TEXT,
  book_no INTEGER PRIMARY KEY ,
  price INTEGER,
);
