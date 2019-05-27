use rumoeba::{start_game};
use rumoeba::ui;

fn main() {
    show_greeting_messages();

    show_game_options();
    loop {
        match choose_game_option() {
            None => {
                ui::show_message("Hm nice try, but I'm afraid I don't know what you mean.")
            }
            Some(option) => {
                match option {
                    GameOption::Start => {
                        start_game();
                        ui::show_message("That was a good ride.")
                    },
                    GameOption::Exit => {
                        ui::show_message("See you soon! Take care of yourself Tiger 😉");
                        break;
                    }
                }
            }
        }
    }
}

fn show_greeting_messages() {
    ui::show_message("Welcome dear visitor in the worlds most ambitious, \
    most violent, most demanding game .. OF .. RUMOEBA!");
    ui::show_message("But first things first. Please humble me with giving your first choice.");
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
    ui::show_message("1) Start RUMOEBA!");
    ui::show_message("2) Exit");
}

fn choose_game_option() -> Option<GameOption> {
    let mut user_input = String::new();
    if std::io::stdin().read_line(&mut user_input).is_err(){
        // logging?
        return Option::None;
    }
    user_input.trim().parse::<GameOption>().ok()
}