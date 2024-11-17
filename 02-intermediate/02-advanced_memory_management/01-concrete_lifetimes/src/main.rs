fn main() {
    #[allow(unused_variables)]
    let r1: &String;
    { // scope begins here
        #[allow(unused_variables)]
        let s = String::from("Hello");
        // error: `s` does not live long enough
        //r1 = &s;
    }
    // out of scope
    // println!("{}", r1);
}
