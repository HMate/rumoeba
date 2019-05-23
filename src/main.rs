

fn main() {
    println!("Welcome dear visitor in the worlds most ambitious, most violent, most demanding game .. OF .. RUMOEBA!");

    println!("But first things first. Please humble me with giving your first choice.");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input);

    println!("Hello, world! {}", user_input);
}
