use jsonwebtoken::{encode,decode,Validation,Algorithm,Header};
use std::time::{SystemTime, UNIX_EPOCH};




#[derive(Debug, Serialize, Deserialize)]
pub struct LoginToken {
    pub reg_no: String,
    pub exp: usize,
}


pub fn jwt_encoder(reg_no:String)->String{
    let claims = LoginToken {
      reg_no: reg_no,
      exp: SystemTime::now()
          .duration_since(UNIX_EPOCH)
          .unwrap()
          .as_secs() as usize
          + 3600,
  };
  let token=Some(encode(&Header::default(), &claims, "secret".as_ref()).unwrap());
  token.unwrap()
}

pub fn jwt_decoder(token:&String)->Option<LoginToken>{
    let token_data=decode::<LoginToken>(token,"secret".as_ref(),&Validation::default()).unwrap();
    Some(token_data.claims)
}
