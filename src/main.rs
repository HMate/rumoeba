fn main() {
    show_greeting_messages();

    show_game_options();
    loop {
        match choose_game_option() {
            None => {
                println!("Hm nice try, but I'm afraid I don't know what you mean.")
            }
            Some(option) => {
                match option {
                    GameOption::Start => println!("That was a good ride."),
                    GameOption::Exit => {
                        println!("See you soon! Take care of yourself Tiger 😉");
                        break;
                    }
                }
            }
        }
    }
}

fn show_greeting_messages() {
    println!("Welcome dear visitor in the worlds most ambitious, most violent, most demanding game .. OF .. RUMOEBA!");
    println!("But first things first. Please humble me with giving your first choice.");
}

enum GameOption {
    Start,
    Exit,
}

impl std::str::FromStr for GameOption {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(GameOption::Start),
            "2" => Ok(GameOption::Exit),
            _ => Err(())
        }
    }
}

fn show_game_options() {
    println!("1) Start RUMOEBA!");
    println!("2) Exit");
}

fn choose_game_option() -> Option<GameOption> {
    let mut user_input = String::new();
    if std::io::stdin().read_line(&mut user_input).is_err(){
        // logging?
        return Option::None;
    }
    user_input.trim().parse::<GameOption>().ok()
}