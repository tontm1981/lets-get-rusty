fn main() {
    // No errors in the following example
    // let player1 = String::from("player 1");
    // let player2 = String::from("player 2");
    // let result = first_turn(player1.as_str(), player2.as_str());
    // println!("{} wins!", result);

    // Here, we have a shorter lifetime in player2, which causes error
    // let mut player1 = String::from("player 1");
    // let result: &str;
    // {
    //     let player2 = String::from("player 2");
    //     result = first_turn(player1.as_str(), player2.as_str());
    // }
    // println!("{} begins!", result);
}

fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        return p1;
    }
    p2
}
