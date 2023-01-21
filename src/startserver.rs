use crate::utils::Settings;
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{web,App, HttpServer};
use actix_web_lab::middleware::from_fn;
use mongodb::Database;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use lib::*;



fn get_cors_policy(setting: &Settings) -> Cors {
    match setting.get_enviorment() {
        "Dev" => Cors::permissive(),
        _ => {
            //set up cors by requirment
            let cors = Cors::default();
            cors
        }
    }
}

//we send connection to databse from main function to the run function
pub fn run(
    settings: Settings,
    listener: TcpListener,
    db: Database,
) -> Result<Server, std::io::Error> {
    //shared reference for database
    let db = Arc::new(Mutex::new(db));

    let server = HttpServer::new(move || {
        let cors = get_cors_policy(&settings);
        App::new().wrap(cors)
        .app_data(db.clone())
        .route("signup",web::post().to(signup))
        .route("/login", web::post().to(login_handler))
        .service(web::scope("/signedin")
        .wrap(from_fn(middleware_wraper))
        .route("/islogined",web::get().to(check_is_token))
    )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
