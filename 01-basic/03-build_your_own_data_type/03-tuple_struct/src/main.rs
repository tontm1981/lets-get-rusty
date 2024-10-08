fn main() {
    // tuples
    let rgb_color = (255, 106, 0);
    println!("RGB color: {}", rgb_color.0);
    let cymk_color = (0, 58, 100, 0);
    println!("CYMK color: {}", cymk_color.3);

    //tuple structs
    struct RGB(i32, i32, i32);
    struct CYMK(i32, i32, i32, i32);

    let color1 = RGB(255, 106, 0);
    println!("RGB color: {}, {}, {}", color1.0, color1.1, color1.2);
    let color2 = CYMK(0, 58, 100, 0);
    println!("CYMK color: {}, {}, {}, {}", color2.0, color2.1, color2.2, color2.3);

    //unit-like structs
    // struct MyStruct;
}
