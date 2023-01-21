use crate::models::Client;
use crate::response_generator;
use crate::utils::{Error as ERROR, ErrorType};
use actix_web::web;
use actix_web::HttpResponse;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
pub use mongodb::bson::{doc, Document};
use mongodb::Database;
use serde::Deserialize;

//input validation on email and password is yet to be implmeneted

//structure to recieve Manager signup Json body
#[derive(Deserialize)]
pub struct Userinfo {
    password: String,
    email: String,
}
pub async fn signup(
    db: web::Data<Database>,
    info: web::Json<Userinfo>,
) -> Result<HttpResponse, ERROR> {
    //initilize an object of magic Crypt
    let mc = new_magic_crypt!("magickey", 256);
    //generate a hased password to store in database
    let hashed_password = mc.encrypt_str_to_base64(info.password.clone());

    //check if person already exits
    match Client::find_one(
        doc! {"$and":[{"email":{"$regex" : &info.email, "$options" : "i"}},{"verified":true}]},
        &db,
    )
    .await
    {
        Ok(data) => match data {
            Some(_data) => {
                return Err(ERROR::new(ErrorType::BadClientData(
                    "Person with same email already exist",
                )));
            }
            None => {}
        },
        Err(_err) => {
            return Err(ERROR::new(ErrorType::InternalServerError("")));
        }
    }

    let user_data = Client::new(info.email.clone(), hashed_password);

    match Client::insert_one(&user_data, &db).await {
        Ok(_) => {
            return Ok(response_generator("", 201 as u16));
        }
        Err(_err) => return Err(ERROR::new(ErrorType::InternalServerError(""))),
    };

    //else push data in database
}
