fn main() {
    let num = 30;
    println!("{} divided by 5 = {}", num, divide_by_5(num));
    println!("{} divided by 3 = {}", num, divide_by_3(num));
}

fn divide_by_5(num: u32) -> u32 {
    num / 5
}

//using return
fn divide_by_3(num: u32) -> u32 {
    if num == 0 {
        // Return early
        return 0;
    }
    num / 3
}
