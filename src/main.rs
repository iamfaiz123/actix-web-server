mod utils;
pub use mongodb::{options::ClientOptions, Client};
use std::net::TcpListener;
pub use utils::_utils::get_configuration;
mod startserver;
pub use startserver::run;


#[tokio::main]
async fn main() -> Result<(), ()> {
    //taking enviorment variables from .yaml files
    let application = get_configuration().unwrap();

    //binding our network to desired port
    let address = format!("{}:{}", application.get_host(), application.get_port());
    let listener = match TcpListener::bind(address) {
        Ok(data) => data,
        Err(err) => {
            print!("unable to run program {err}");
            return Err(());
        }
    };

    //defining connection to database
    let mut client_options = match ClientOptions::parse(application.get_database_url()).await {
        Ok(data) => data,
        Err(err) => {
            print!("unable to run program {err}");
            return Err(());
        }
    };

    client_options.app_name = Some(application.get_database_name().to_string());
    let client = Client::with_options(client_options).expect(" ");
    let db = client.database(application.get_database_name());

    //external function to start server
    let _a = run(application, listener, db).unwrap().await;
    Ok(())
}
