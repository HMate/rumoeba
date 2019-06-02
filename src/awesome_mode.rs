extern crate console;
use std::thread;
use std::time::Duration;
use console::Term;

pub fn start_awesome() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    term.write_line("Hello World!").unwrap();
    thread::sleep(Duration::from_millis(2000));
    term.move_cursor_up(1).unwrap();
    term.clear_line().unwrap();
    thread::sleep(Duration::from_millis(3000));
}

