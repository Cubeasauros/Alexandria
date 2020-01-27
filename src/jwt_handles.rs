use jsonwebtoken::{decode,Validation,Algorithm};
use super::json_handle::LoginToken;
pub fn jwt_encoder(){

}

pub fn jwt_decoder(token:String)->Option<LoginToken>{
    let token_data=decode::<LoginToken>(&token,"secret".as_ref(),&Validation::default()).unwrap();
    Some(token_data.claims)
}
