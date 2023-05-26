//! Contains a function that checks whether all required environment variables are set.

use std::env;

use ansi_term::Color;
use lazy_static::lazy_static;

use crate::errors::StaccError;

lazy_static! {
    /// All environment variables required to run the API.
    static ref ENVIRONMENT_VARIABLES: Vec<&'static str> = vec![
        "MONGO_PASSWORD",
        "MONGO_URI",
        "MONGO_USER",
        "STACC_BACKGROUNDS_COLLECTION_NAME",
        "STACC_DATABASE",
        "STACC_POSTS_COLLECTION_NAME",
        "STACC_VISITORS_COLLECTION_NAME",
    ];
}

/// Check for all environment variables.
pub fn run_environment_checks() -> Result<(), StaccError> {
    println!(
        "{}",
        Color::Yellow
            .bold()
            .paint("❗️ RUNNING ENVIRONMENT VARIABLES CHECK")
    );

    for variable in ENVIRONMENT_VARIABLES.iter() {
        print!("🧐 Checking for \"{variable}\"... ");

        if let Err(error) = env::var(variable) {
            println!("{}", Color::Red.bold().paint("⭕️‼️ "));

            return Err(StaccError::EnvironmentError(error));
        }

        println!("{}", Color::Green.bold().paint("OK"));
    }

    println!(
        "{}",
        Color::Green
            .bold()
            .paint("💯 ALL ENVIRONMENT VARIABLES ARE SET")
    );

    Ok(())
}
