use rand::prelude::*;
use database::Status;
pub use crate::auth_utils::models::Credentials;

mod database;
mod auth_utils;

pub fn authenticate(creds: Credentials) {
    let timeout = thread_rng().gen_range(100..500);
    println!("Timeout: {}", timeout);
    if let Status::Connected = database::connect_to_database() {
        println!("Connected to database");
        auth_utils::login(creds);
    }
}

pub fn logout() {
    auth_utils::logout();
    if let Status::Interrupted = database::close_connection() {
        println!("Disconnected from database");
    }
}