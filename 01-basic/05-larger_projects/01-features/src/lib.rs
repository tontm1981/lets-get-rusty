pub fn draw_line(x: i32, y: i32) {
    println!("root :: draw_line");
    println!("{x}, {y}");
}

#[cfg(feature = "color")]
pub mod color {
    pub use rgb::RGB;
    
    pub fn draw_line(x: i32, y: i32, color: &RGB<u16>) {
        println!("mod color :: Drawing line");
        println!("{x}, {y}");
        println!("{color:?}")
        // draw line with color
    }
}

#[cfg(feature = "shapes")]
pub mod shapes {
    use serde::{Serialize, Deserialize};
    use rgb::RGB;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rectangle {
        pub color: RGB<u16>,
        pub width: u32,
        pub height: u32,
    } 
}