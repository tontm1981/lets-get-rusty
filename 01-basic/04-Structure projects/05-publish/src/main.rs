use auth_service::{authenticate, logoff, Credentials};

fn main() {
    let credentials = Credentials {
        username: "teste".to_string(), 
        password: "123456".to_string() 
    };
    authenticate(credentials);
    logoff();
}