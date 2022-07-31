#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

use std::env;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod app;
mod constants;
mod error;
mod middleware;
mod routes;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start conduit server...");
    // std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();

    let state = {
        let pool = utils::db::establish_connection();
        middleware::state::AppState { pool }
    };

    let port = env::var(constants::env_key::PORT).unwrap_or_else(|_| "8080".to_string());

    let res = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(state.clone()))
            .wrap(middleware::cors::cors())
            .wrap(middleware::auth::Authentication)
            .configure(routes::api)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await;

    match res {
        Ok(_) => {
            Ok(())
        },
        Err(e) => {
            eprintln!("{:?}", e);
            Err(e)
        }
    }
}
