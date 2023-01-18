#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age,u32), // todo!("Move `mileage: u32` field into `age` field - a tuple with two fields: an `Age` enum, u32");
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

fn car_quality (miles: u32) -> (Age, u32) {
    let mut quality: (Age, u32)= (Age::New, 0);

    if miles > 0 {
        quality = (Age::Used, miles);
    }

    quality
}
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

fn main() {
    let car_1 = car_factory(String::from("White"), Transmission::Automatic, true, 100);
    let car_2 = car_factory(String::from("Yellow"), Transmission::Manual, true, 0);
    let car_3 = car_factory(String::from("Black"), Transmission::SemiAuto, true, 35);
    println!(" {:?}\n {:?}\n {:?}", car_1, car_2, car_3);

    let colors = ["Blue", "Green", "Red", "Silver"];
    let mut car: Car;
    let mut engine = Transmission::Manual;

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!("Car order 1: {:?}, hard top = {}, {:?}, {} {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
    // Car order #2: New, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
    // Car order #3: New, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
}
