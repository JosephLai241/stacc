//! Contains a function that checks whether all required environment variables are set.

use std::{
    collections::HashSet,
    env::{self, VarError},
};

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
        "STACC_DOMAIN",
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
            .paint("❗️ RUNNING ENVIRONMENT VARIABLES CHECK...")
    );

    let loaded_variables: HashSet<String> = env::vars().map(|(key, _value)| key).collect();
    let required_variables: HashSet<String> = ENVIRONMENT_VARIABLES
        .iter()
        .map(|variable| variable.to_string())
        .collect();

    if !required_variables.is_subset(&loaded_variables) {
        let missing_keys: Vec<String> = required_variables
            .difference(&loaded_variables)
            .cloned()
            .collect();

        println!(
            "{} Missing the following keys: {:#?}",
            Color::Red.bold().paint("⭕️‼️"),
            missing_keys
        );

        return Err(StaccError::EnvironmentError(VarError::NotPresent));
    }

    println!(
        "{}",
        Color::Green
            .bold()
            .paint("💯 ALL ENVIRONMENT VARIABLES ARE SET")
    );

    Ok(())
}
