use std::rc::Rc;

#[derive(Debug)]
struct Database {}

#[derive(Debug)]
struct AuthService{
    #[allow(dead_code)]
    db: Rc<Database>,
}

#[derive(Debug)]
struct ContentService{
    #[allow(dead_code)]
    db: Rc<Database>,
}

fn main() {
    // Reference Counter (RC) allow us to have a shared ownership
    let db = Rc::new(Database{});

    // It's possible to use the following way, but it can lead to misunderstanding about what will be cloned
    // let auth_service = AuthService{db: db.clone()};
    // let content_service = ContentService{db: db.clone()};

    // So, it's preferable to use the following way
    let auth_service = AuthService{db: Rc::clone(&db)};
    let content_service = ContentService{db: Rc::clone(&db)};

    println!("{:?}", auth_service);
    println!("{:?}", content_service);
}