// for auth of request use
// middleware::new(req).auth();
use actix_web::body::MessageBody;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    HttpMessage,
};
use actix_web_lab::middleware::Next;
use jwt_simple::prelude::*;
use magic_crypt::new_magic_crypt;
use magic_crypt::MagicCryptTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
// role can be User, Admin
pub struct CustomClaim {
    email: String,
}

#[derive(serde::Deserialize)]
pub struct Key {
    jwt_encryption_key: String,
}

pub struct Middleware {
    pub token: String,
}

impl Middleware {
    pub fn new(token: String) -> Middleware {
        Middleware { token }
    }

    //middleware struct returns enum
    //ex. (false,None,None) or (true,<email>,Role,Security,peer_ip) based on varification
    pub fn auth(self) -> (bool, Option<String>) {
        //generate key
        let mut key = config::Config::default();
        //these unwrap panic when the program starts, better to close than handle
        key.merge(config::File::with_name("key")).unwrap();
        let key: Key = key.try_into().unwrap();
        let key = HS256Key::from_bytes(&key.jwt_encryption_key.as_bytes());
        let token = self.token.to_string();
        let mut options = VerificationOptions::default();

        //set time fine upto 15mins
        options.accept_future = true;

        let claims = key.verify_token::<CustomClaim>(&token, Some(options));

        //extract mail from token
        let email = match claims {
            //if token verifiy
            Ok(claim) => claim.custom.email,
            //if failed
            Err(_err) => return (false, None),
        };

        //decrypt the email and return
        let mcrypt = new_magic_crypt!("magickey", 256);
        let user_email = mcrypt.decrypt_base64_to_string(email).unwrap();

        return (true, Some(user_email));
    }
}
//struct to pass to actix body
#[derive(Deserialize, Clone)]
pub struct Auth {
    pub email: std::string::String,
}

//middleware to use
pub async fn middleware_wraper(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    //extract head and body
    let (head, _body) = req.parts_mut();

    let token = match head.headers().get("Authorization") {
        Some(data) => data.to_str().unwrap().to_string(),
        None => return Err(actix_web::error::ErrorBadRequest("token not found")),
    };

    //validate token
    let validate = Middleware::new(token.clone()).auth();
    // if token not varified
    if !validate.0 {
        return Err(actix_web::error::ErrorBadRequest("cant validate"));
    }
    let email = validate.1.unwrap();
    let auth = Auth { email };
    //create data to pass into the handler
    //pass req to handler
    req.extensions_mut().insert(auth);
    //pass this to handler
    next.call(req).await
}
