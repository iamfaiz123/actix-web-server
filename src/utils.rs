use actix_web::error;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::json;
use std::clone::Clone;
use std::fmt;

#[derive(serde::Deserialize)]
pub struct Settings {
    port: u16,
    host: String,
    data_base_setting: DataBase,
    enviorment: String,
}

impl Settings {
    pub fn get_port(&self) -> u16 {
        self.port
    }
    pub fn get_host(&self) -> &str {
        &self.host
    }
    pub fn get_enviorment(&self) -> &str {
        &self.enviorment
    }

    pub fn get_database_name(&self) -> &str {
        &self.data_base_setting.name
    }
    pub fn get_database_url(&self) -> &str {
        &self.data_base_setting.url
    }
}

impl Clone for Settings {
    fn clone(&self) -> Self {
        Settings {
            port: self.get_port(),
            host: self.get_host().to_string(),
            enviorment: self.get_enviorment().to_string(),
            data_base_setting: DataBase {
                url: self.get_database_url().to_string(),
                name: self.get_database_name().to_string(),
            },
        }
    }
}

#[derive(serde::Deserialize)]
pub struct DataBase {
    pub url: String,
    pub name: String,
}

pub mod _utils {
    use super::*;
    pub fn get_configuration() -> Result<Settings, config::ConfigError> {
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name("config"))?;
        settings.try_into()
    }
    pub fn response_generator(message: &str, statuscode: u16) -> actix_web::HttpResponse {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("X-Hdr", "sample"))
            .status(StatusCode::from_u16(statuscode).unwrap())
            .json(json!({ "message": message }));
    }
}

#[derive(Debug)]
pub enum ErrorType {
    InternalServerError(&'static str),
    BadRequest(&'static str),
    BadClientData(&'static str),
    Conflict(&'static str),
    MethodNotAllowed(&'static str),
}

#[derive(Debug)]
pub struct Error {
    cause: ErrorType,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.cause {
            ErrorType::InternalServerError(data) => {
                write!(f, "{}", data)
            }
            ErrorType::BadRequest(data) => {
                write!(f, "{}", data)
            }
            ErrorType::BadClientData(data) => {
                write!(f, "{}", data)
            }
            ErrorType::Conflict(data) => {
                write!(f, "{}", data)
            }
            ErrorType::MethodNotAllowed(data) => {
                write!(f, "{}", data)
            }
        }
    }
}

impl Error {
    pub fn new(cause: ErrorType) -> Error {
        Error { cause }
    }
}

impl error::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self.cause {
            ErrorType::InternalServerError(_e) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::BadClientData(_e) => StatusCode::BAD_REQUEST,
            ErrorType::BadRequest(_e) => StatusCode::GATEWAY_TIMEOUT,
            ErrorType::Conflict(_e) => StatusCode::CONFLICT,
            ErrorType::MethodNotAllowed(_e) => StatusCode::METHOD_NOT_ALLOWED,
        }
    }
}
