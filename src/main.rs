// Declare Car struct to describe vehicle with four named fields
struct Car { color: String, transmission: Transmission, convertible: bool, mileage: u32 }

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission { Manual, SemiAuto, Automatic }

fn is_convertible(convertible: bool) -> String {
    if convertible {
        return "convertible".to_string();
    }
    "no convertible".to_string()
}

fn main() {
    //exe 1
    let t_automatic = Transmission::Automatic;
    let t_manual = Transmission::Manual;
    let t_semi_auto = Transmission::SemiAuto;
    println!("transmission types: {:#?}, {:#?}, {:#?}", t_automatic, t_manual, t_semi_auto);

    let car = Car { transmission: t_automatic, color: "Red".to_string(), convertible: true, mileage: 20};

    println!("the {} is a {} {:#?} car with {} mileage", is_convertible(car.convertible), car.color, car.transmission, car.mileage);

    //exe 2
    let new_car = car_factory(String::from("Blue"), Transmission::SemiAuto, false);
    println!("new car {} {:#?} {} {}", new_car.color, new_car.transmission, new_car.convertible, new_car.mileage);
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    return Car {color, convertible, transmission, mileage: 35};
}
