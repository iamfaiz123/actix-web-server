use crate::{models, response_generator, Token};
use models::Client;
//functions and struct
use crate::utils::{Error as ERROR, ErrorType};
use actix_web::{web, HttpResponse};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
pub use mongodb::bson::{doc, Document};
use mongodb::Database;
use serde::Deserialize;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use serde_json::json;




#[derive(Deserialize)]
pub struct Userinfo {
    password: String,
    email: String,
}
//start of signup handler
pub async fn login_handler(
	db: web::Data<Database>,
	info:web::Json<Userinfo>,
) -> Result<HttpResponse, ERROR> {

     let email = info.email.clone();

     let hashpassword = match Client::find_one(doc!{"email":&email},&db).await{
        Ok(data)=>{
            match data{
                Some(data)=>{
                    data.hash_password
                }
                None=>{
                    //if email does not exist in database
                    return Ok(response_generator("email or password is incorrect", 201))
                }
            }
        }
        Err(_err)=>{
            return Err(ERROR::new(ErrorType::InternalServerError("")));
        }
     };

    let mc = new_magic_crypt!("magickey", 256);
    //generate a hased password to store in database
    let provided_hashed_password = mc.encrypt_str_to_base64(info.password.clone());

    if !hashpassword.eq(&provided_hashed_password){
        return Ok(response_generator("email or password is incorrect", 201))
    }
    else{
        
        let token = Token::create_token(info.email.clone());
        return Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .append_header(("X-Hdr", "sample"))
            .status(StatusCode::from_u16(201).unwrap())
            .json(json!({ "Auth": token})));
    }
}
