pub fn login(_creds: models::Credentials) {
    crate::database::get_user();
}

pub fn logout() {
    println!("Logged out");
}

pub mod models;