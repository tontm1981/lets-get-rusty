use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Database {
    max_connections: u32,
}

#[derive(Debug)]
struct AuthService{
    #[allow(dead_code)]
    db: Rc<RefCell<Database>>,
}

#[derive(Debug)]
struct ContentService{
    #[allow(dead_code)]
    db: Rc<RefCell<Database>>,
}

fn main() {
    // Reference Counter (RC) allow us to have a shared ownership. It accepts only immutable shared ownership
    let db = Rc::new( RefCell::new( Database{ max_connections: 100 }) );

    // It's possible to use the following way, but it can lead to misunderstanding about what will be cloned
    // let auth_service = AuthService{db: db.clone()};
    // let content_service = ContentService{db: db.clone()};

    // So, it's preferable to use the following way
    let auth_service = AuthService{db: Rc::clone(&db)};
    let content_service = ContentService{db: Rc::clone(&db)};

    // Even declaring `db` as muttable, due to fact that RC allows only immutable shared ownership, it's not possible to change properties values
    // To get around Rust borrowing rules and use mutability and shared ownership at same time, we must to use RC in combination with RefCell
    db.borrow_mut().max_connections = 200;
    println!("{:?}", auth_service);
    println!("{:?}", content_service);

    // Even borrowing var as mutable, we need to declare var as mutable too
    let mut r1 = db.borrow_mut();
    // Even working with mutable shared ownership, only ONE mutable reference is expected.
    // let r2 = db.borrow_mut(); // Error: already borrowed

    r1.max_connections = 300;
}