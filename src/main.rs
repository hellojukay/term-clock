use chrono::Local;
use console::Term;
use figlet_rs::FIGfont;
use std::{io, usize};
use std::thread::sleep;
use std::time::Duration;

struct Screen {
    buffer: usize,
    term: Term,
}
impl Screen {
    pub fn write(self: &mut Self, str: &str) {
        let len = str.as_bytes().len();
        println!("{}", str);
        self.buffer = len;
    }
    pub fn clear(self : &Self) {
        self.term.move_cursor_to(0, 0).unwrap();
        self.term.clear_chars(self.buffer).unwrap();
    }
    pub fn clean_all(self:  &Self){
        self.term.clear_screen().unwrap();
    }
    pub fn new() -> Self {
        let screen = Screen{
            term: Term::stdout(),
            buffer: 0 as usize,
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
        screen.clear();
    }
}
