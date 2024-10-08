pub fn login(_creds: models::Credentials) {
    crate::database::get_user();
}

pub fn logout() {
    crate::database::close_connection();
    println!("Logged out");
}

pub mod models;