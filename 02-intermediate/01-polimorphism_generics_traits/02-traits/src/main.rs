#[allow(dead_code)]
trait Park {
    fn park(&self);
}

#[allow(dead_code)]
trait Paint {
    fn paint(&self, color: String) {
        println!("Painting the vehicle {}", color);
    }
}

#[allow(dead_code)]
struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("Painting the house {}", color);
    }
}

#[allow(dead_code)]
struct VehicleInfo {
    maker: String,
    model: String,
    year: u32,
}

#[allow(dead_code)]
struct Car {
    info: VehicleInfo,
}

impl Park for Car {
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

impl Park for Truck {
    fn park(&self) {
        println!("Parking the truck");
    }
}

impl Paint for Truck {}

fn main() {

}