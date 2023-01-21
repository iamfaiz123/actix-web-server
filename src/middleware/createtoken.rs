use jwt_simple::prelude::*;
use magic_crypt::new_magic_crypt;
use magic_crypt::MagicCryptTrait;
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize)]
pub struct Key {
    jwt_encryption_key: String,
}

pub struct Token;

impl Token {
    //function can take ownership
    pub fn create_token(email: String) -> String {
        //custom claim , we're using email, but it is encrypted
        #[derive(Serialize, Deserialize)]
        pub struct CustomClaim {
            email: String,
        }

        let mut key = config::Config::default();
        //these unwrap panic when the program starts, better to close than handle
        key.merge(config::File::with_name("key")).unwrap();
        let key: Key = key.try_into().unwrap();

        // same key will be used for varification

        let key = HS256Key::from_bytes(&key.jwt_encryption_key.as_bytes());

        //encrypt email before creating jwt token
        let mcrypt = new_magic_crypt!("magickey", 256);
        let encrypted_email = mcrypt.encrypt_str_to_base64(email);

        let customclaim = CustomClaim {
            email: encrypted_email,
        };
        //duration of the time token will be valid for
        let time = Duration::from_hours(512);

        let claim = Claims::with_custom_claims(customclaim, time);

        let token = key.authenticate(claim).expect("fail to create token");

        //return token string
        token
    }
}
