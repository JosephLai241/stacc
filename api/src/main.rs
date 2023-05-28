//! The API for the `stacc`.

use std::sync::Mutex;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use ansi_term::{Color, Style};
use dotenv::dotenv;

use utils::mongo::Mongo;

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
        let mongo = Data::new(Mutex::new(
            Mongo::init()
                .await
                .expect("COULD NOT INSTANTIATE A NEW MONGODB CLIENT INSTANCE!"),
        ));

        HttpServer::new(move || {
            App::new().app_data(mongo.clone()).service(
                web::scope("api")
                    .service(routes::misc::get_background_gif)
                    .service(
                        web::scope("/blog")
                            .service(routes::posts::get_all_posts)
                            .service(routes::posts::get_single_post),
                    ),
            )
        })
        .bind(("127.0.0.1", 8081))
        .expect("FAILED TO BIND TO THE SOCKET ADDRESS")
        .run()
        .await
        .expect("API FAILED TO START");
    }
}
