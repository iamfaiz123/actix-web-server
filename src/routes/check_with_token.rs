use crate::Auth;
use actix_web::HttpResponse;
use crate::utils::Error as ERROR;
use actix_web::web;
use crate::response_generator;

pub async fn check_is_token(
	_credentials: Option<web::ReqData<Auth>>,
) -> Result<HttpResponse, ERROR> {
    return Ok(response_generator("logined",201 as u16));
}

