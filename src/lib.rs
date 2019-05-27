pub mod ui;

pub fn start_game() {
    ui::show_message("Pick a board size!");
    let board_size: u32;
    loop {
        let mut user_input = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut user_input) {
            continue;
        }
        if let Ok(parsed_number) = user_input.trim().parse::<u32>() {
            if 2 < parsed_number && parsed_number < 25{
                board_size = parsed_number;
                break;
            }
        }
    }

    ui::show_message(&format!("Let the game begin! {}", board_size));
    //loop {}
}



