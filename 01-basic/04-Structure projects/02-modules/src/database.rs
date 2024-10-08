pub enum Status {
    Connected,
    Interrupted,
}

pub fn connect_to_database() -> Status {
    Status::Connected
}

pub fn close_connection() -> Status {
    Status::Interrupted
}

pub fn get_user() {
    println!("Got user");
}