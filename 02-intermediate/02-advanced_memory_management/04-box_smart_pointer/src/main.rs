#[allow(dead_code)]
trait UIComponent {
    fn render(&self) {
        println!("Rendering UI component");
    }
}

struct Button {
    text: String,
}

impl UIComponent for Button {}

#[allow(dead_code)]
struct Container {
    name: String,
    child: Box<Container>,
}

fn main() {
    // Following var will be allocated in the stack
    let button_a = Button { text: "Button A".to_string() };

    // The following var will be allocated in the heap
    let button_b = Box::new(Button { text: "Button B".to_string() });

    // Box Smart Pointer usecases:
    // 1. Ownership
    //    - Avoid large data structures to be copied
    //    - Avoid large data structures to be moved
    // 2. Polymorphism
    //    - Use a single interface to represent different types
    // 3. Dynamic Dispatch
    //    - Use a single interface to represent different types

    // The `button_a` ownership be transfered to `button_c`
    // This is because `button_a` is a simple variable
    // and its data is stored in the stack 
    let button_c = button_a;
    println!("{}", button_c.text);
    // println!("{}", button_a.text);

    // In the following case, just the Box ownership will be transfered to button_d
    let button_d = button_b;
    println!("{}", button_d.text);

    #[allow(unused_variables)]
    let components = vec![
        Box::new(button_c),
        button_d
    ];
}