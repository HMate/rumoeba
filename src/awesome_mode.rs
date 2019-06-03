extern crate console;
use console::Term;
use crate::xogame;


pub fn start_awesome() {
    let term = Term::stdout();

    let dimension_range = xogame::MIN_BOARD_SIZE..xogame::MAX_BOARD_SIZE;
    let mut selected_index = 0;

    let mut waiting_enter = true;
    while waiting_enter {
        term.clear_screen().unwrap();
        term.write_line("The dimensions Mason! What are they?").unwrap();

        let mut dim_chooser_line = String::with_capacity(dimension_range.len() * 4);
        for (index, dim) in dimension_range.clone().enumerate() {
            if index == selected_index {
                dim_chooser_line.push_str(&format!("*{:}*", dim));
            } else {
                dim_chooser_line.push_str(&format!("{:^4}", dim));
            }
        }
        term.write_line(&dim_chooser_line).unwrap();

        match term.read_key().unwrap() {
            console::Key::Enter => waiting_enter = false,
            console::Key::ArrowLeft => {
                if selected_index == 0 {
                    selected_index = dimension_range.len()-1;
                } else {
                    selected_index -= 1;
                }
            },
            console::Key::ArrowRight => {
                selected_index += 1;
                if selected_index >= dimension_range.len() {
                    selected_index = 0;
                }
            },
            _ => ()
        };
    }

    term.clear_line().unwrap();
}

