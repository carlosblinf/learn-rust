fn main() {
    // Declare array, initialize all values, compiler infers length = 7
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    
    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];

    println!("{:#?} {:#?}", days, bytes);
}
