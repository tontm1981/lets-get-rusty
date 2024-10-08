pub enum Status {
    Connected,
    Interrupted,
}

pub fn connect_to_database() -> Status {
    return Status::Connected
}

pub fn close_connection() -> Status {
    return Status::Interrupted
}

pub fn get_user() {
    println!("Got user");
}