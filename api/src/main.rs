//! The API for the `stacc`.

use std::env;

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use ansi_term::{Color, Style};
use dotenv::dotenv;
use env_logger::Env;

use utils::{environment::EnvironmentVariables, mongo::Mongo};

mod errors;
mod middleware;
mod models;
mod routes;
mod utils;

#[actix_web::main]
async fn main() {
    println!(
        "\n{}",
        Style::default().bold().paint("🚀 Initializing API.")
    );

    println!("📃 Reading environment variables from the `.env` file.");
    dotenv().ok();

    if let Err(error) = utils::checks::run_environment_checks() {
        println!("{}", Color::Red.bold().paint(error.to_string()));
    } else {
        // Instantiate the API logger.
        env::set_var("RUST_LOG", "debug");
        env_logger::init_from_env(Env::new().default_filter_or("debug"));

        let mongo = Data::new(
            Mongo::init()
                .await
                .expect("COULD NOT INSTANTIATE A NEW MONGODB CLIENT INSTANCE!"),
        );

        HttpServer::new(move || {
            App::new()
                .app_data(mongo.clone())
                .service(
                    web::scope("api")
                        .service(routes::misc::get_background_gif)
                        .service(routes::misc::story)
                        .service(
                            web::scope("/blog")
                                .service(routes::posts::get_all_posts)
                                .service(routes::posts::get_single_post),
                        ),
                )
                .wrap(
                    Cors::default()
                        .allowed_header(header::CONTENT_TYPE)
                        .allowed_methods(vec!["GET"])
                        .allowed_origin(
                            &EnvironmentVariables::StaccDomain
                                .env_var()
                                .unwrap_or("UNKNOWN".to_string()),
                        ),
                )
                .wrap(Logger::default())
        })
        .bind(("127.0.0.1", 8081))
        .expect("FAILED TO BIND TO THE SOCKET ADDRESS")
        .run()
        .await
        .expect("API FAILED TO START");
    }
}
