#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn car_quality(miles: u32) -> (Age, u32) {
    let mut quality: (Age, u32) = (Age::New, 0);

    if miles > 0 {
        quality = (Age::Used, miles);
    }

    quality
}
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    if roof {
        if miles == 0 {
            println!(
                "Build a new car: {:?}, {}, Hard top, {} miles",
                motor, color, miles
            );
        } else {
            println!(
                "Prepare a used car: {:?}, {}, Hard top, {} miles",
                motor, color, miles
            );
        }
    } else if miles > 0 {
        println!(
            "Prepare a used car: {:?}, {}, Convertible, {} miles",
            motor, color, miles
        );
    } else {
        println!(
            "Prepare a used car: {:?}, {}, Hard top, {} miles",
            motor, color, miles
        );
    }
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

fn main() {
    let mut car: Car;
    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from("Orange"), Transmission::Manual, true, 0);
    println!(
        "Car order 1: {:?}, hard top = {}, {:?}, {} {} miles\n",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #2: Used, Semi-automatic, Convertible
    car = car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles\n",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #3: Used, Automatic, Hard top
    car = car_factory(String::from("White"), Transmission::Automatic, true, 3000);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles\n",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
}
