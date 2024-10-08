trait Park {
    fn park(&self);
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

impl Park for Car {
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

impl Park for Truck {
    fn park(&self) {
        println!("Parking the truck");
    }
}

impl Paint for Truck {}

fn main() {

}