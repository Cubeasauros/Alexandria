
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde;
extern crate jsonwebtoken;
pub mod json_handle;
pub mod jwt_handles;
pub mod database_handle;
pub mod sq_lite_test;
use rocket_contrib::databases::mysql;
#[database("codex")]
pub struct CodexDb(mysql::Conn);
