
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde;
#[macro_use] extern crate diesel;

extern crate jsonwebtoken;

use rocket_contrib::databases::mysql;

pub mod database_handle;


/*
//scheduled for deletion
#[database("codex")]
pub struct CodexDb(mysql::Conn);
*/

#[database("codex_pg")]
pub struct CodexPg(diesel::PgConnection);
