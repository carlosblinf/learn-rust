struct Student {
    name: String,
    level: u8,
    remote: bool,
}

// Tuple struct with data types only
struct Grades(char, char, char, char, f32);


// Define a tuple struct
#[derive(Debug)]
struct KeyPress(String, char);

// Define a classic struct
#[derive(Debug)]
struct MouseClick { x: i64, y: i64}

// Define the WebEvent enum variants to use the data from the structs
// and a boolean type for the page Load variant
#[derive(Debug)]
enum WebEvent {  WELoad(bool), WEKeys(KeyPress), WEClick(MouseClick)
}


fn main() {
    //structs
    let user_1 = Student {name: String::from("Carlos"), level: 10, remote: true};
    let mark_1 = Grades('A', 'B', 'A', 'B', 5.3);

    let user_2 = Student {name: "Marcos".to_string(), remote: false, level:5};
    let mark_2 = Grades('B', 'C', 'D', 'A', 7.4);

    println!("{}, level {}, remote {}. Grades {}, {}, {}, {}. Average: {}", user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    println!("{}, level {}, remote {}. Grades {}, {}, {}, {}. Average: {}", user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);


    //enums:
    let we_load = WebEvent::WELoad(true);

    let click = MouseClick {x: 100, y: 100};
    println!("Mouse click location: {}, {}", click.x, click.y);
    let we_click = WebEvent::WEClick(click);

    let key = KeyPress("Ctrol+".to_string(), 'N');
    println!("\nKeys pressed: {}{}", key.0, key.1);
    let we_key = WebEvent::WEKeys(key);

    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);

}
