enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    },
}

impl Command {
    fn serialize(&self) -> String {
        match self {
            Command::Undo => format!("{{\"cmd\":\"undo\"}}"),
            Command::Redo => String::from("{\"cmd\":\"redo\"}"),
            Command::AddText(text) => format!("{{\"cmd\":\"add_text {}\"}}", text),
            Command::MoveCursor(x, y) => format!("{{\"cmd\":\"move_cursor {} {}\"}}", x, y),
            Command::Replace { from, to } => format!("{{\"cmd\":\"replace {} {}\"}}", from, to),
        }
    }
}

fn main() {
    let cmd = Command::Undo;
    let cmd1 = Command::AddText("test".to_string());
    let cmd2 = Command::MoveCursor(25, 0);
    let cmd3 = Command::Replace {
        from: "foo".to_string(),
        to: "bar".to_string(),
    };
    let cmd4 = Command::Redo;
    println!("{}", cmd.serialize());
    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());
    // let age: i32 = 35;
    // match age {
    //     1 => println!("Happy first birthday"),
    //     2..=12 => println!("Have a nice and happy childhood"),
    //     13 ..= 19 => println!("Teenager"),
    //     d if d >=120 => println!("You are Matusaleem"),
    //     x => println!("You are an adult with {x} years old"),
    // }
}
