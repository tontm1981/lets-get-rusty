trait Vehicle: Paint {
    fn park(&self);

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    maker: String,

    #[allow(dead_code)]
    model: String,

    #[allow(dead_code)]
    year: u32,
}

struct Car {
    #[allow(dead_code)]
    info: VehicleInfo,
}

impl Vehicle for Car {
    fn park(&self) {
        println!("Parking the car");
    }
}

impl Paint for Car {}

#[allow(dead_code)]
struct Truck {
    info: VehicleInfo,
}

impl Truck {
    #[allow(dead_code)]
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

    let object = create_paintable_object(true);
    paint_blue(object.as_ref());

    #[allow(unused_variables)]
    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];
}

#[allow(dead_code)]
fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_string());
}

fn paint_blue(object: &dyn Paint) {
    object.paint("blue".to_string());
}

fn paint_green<T>(object: &T) where T: Paint {
    object.paint("green".to_string());
}

#[allow(dead_code)]
fn paint_vehicle_green<T>(object: &T) where T: Vehicle {
    object.paint("green".to_string());
}

#[allow(dead_code)]
fn paint_vehicle_red(object: &impl Vehicle) {
    object.paint("red".to_string());
}

fn create_paintable_object(is_vehicle: bool) -> Box<dyn Paint> {
    if is_vehicle {
        Box::new(Car {
            info: VehicleInfo {
                maker: "Toyota".to_string(),
                model: "Corolla".to_string(),
                year: 2020,
            },
        })
    } else {
        Box::new(House {})
    }
}