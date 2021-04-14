use chrono::Local;
use console::Term;
use figlet_rs::FIGfont;
use std::thread::sleep;
use std::{io};

use std::time::Duration;

struct Screen {
    term: Term,
}
impl Screen {
    pub fn write(self: &mut Self, str: &str) {
        self.term.move_cursor_to(0, 0).unwrap();
        self.clear();
        self.term.move_cursor_to(0, 0).unwrap();
        println!("{}", str);
    }
    pub fn clear(self : &Self) {
        self.term.move_cursor_to(0, 0).unwrap();
        if let Some((w, h)) = term_size::dimensions() {
            let mut s = String::from("");
            for _ in 0..h {
                s.push_str(" ".repeat(w).as_str());
                s.push_str("\n");
            }
            println!("{}",s);
        } else {
            panic!("can not get terminal size");
        }
    }
    pub fn clean_all(self:  &Self){
        self.term.clear_screen().unwrap();
    }
    pub fn new() -> Self {
        let screen = Screen{
            term: Term::stdout(),
        };
        screen.term.hide_cursor().unwrap();
        screen
    }
}
fn main() -> io::Result<()> {
    let mut  screen = Screen::new();
    screen.clean_all();
    loop {
        let date = Local::now();
        let standard_font = FIGfont::standand().unwrap();
        let figure = standard_font.convert(date.format("%H:%M:%S").to_string().as_str());
        assert!(figure.is_some());
        screen.write(figure.unwrap().to_string().as_str());
        // we sleep for 2 seconds
        sleep(Duration::new(1, 0));
    }
}
