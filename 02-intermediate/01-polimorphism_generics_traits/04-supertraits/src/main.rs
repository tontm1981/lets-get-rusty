trait Vehicle: Paint {
    fn park(&self);

    fn get_default_color() -> String {
        "black".to_string()
    }
}

trait Paint {
    fn paint(&self, color: String) {
        println!("Painting the vehicle {}", color);
    }
}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("Painting the house {}", color);
    }
}

struct VehicleInfo {
    maker: String,
    model: String,
    year: u32,
}

struct Car {
    info: VehicleInfo,
}

impl Vehicle for Car {
    fn park(&self) {
        println!("Parking the car");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn unload(&self) {
        println!("Unloading the truck");
    }
}

impl Vehicle for Truck {
    fn park(&self) {
        println!("Parking the truck");
    }
}

impl Paint for Truck {}

fn main() {
    let car = Car {
        info: VehicleInfo {
            maker: "Toyota".to_string(),
            model: "Corolla".to_string(),
            year: 2020,
        },
    };
    car.park();
    car.paint("coffee".to_owned());

    let house = House{};
    paint_green(&house);

    let object = create_paintable_object();
    paint_blue(&object);
}

fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_string());
}

fn paint_blue(object: &impl Paint) {
    object.paint("blue".to_string());
}

fn paint_green<T>(object: &T) where T: Paint {
    object.paint("green".to_string());
}

fn paint_vehicle_green<T>(object: &T) where T: Vehicle {
    object.paint("green".to_string());
}

fn paint_vehicle_red(object: &(impl Vehicle)) {
    object.paint("red".to_string());
}

fn create_paintable_object() -> impl Paint {
    House {}
}