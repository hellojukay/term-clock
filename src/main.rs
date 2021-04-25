use chrono::Local;
use figlet_rs::FIGfont;
use std::thread::sleep;
use std::{io};

use std::time::Duration;
use std::io::{Write};
use terminal::{Clear, Action};

struct Screen {
}

impl Screen {
    pub fn write(&mut self, str: &str) {
        println!("{}", str);
        std::io::stdout().flush().unwrap();
    }
    pub fn clear(&self) {
        let t = terminal::stdout();
        t.act(Action::ClearTerminal(Clear::All)).unwrap();
        t.batch(Action::MoveCursorTo(0, 0)).unwrap();
    }
    pub fn new() -> Self {
        Screen{}
    }
}

fn main() -> io::Result<()> {
    let mut  screen = Screen::new();
    let standard_font = FIGfont::standand().unwrap();
    loop {
        let date = Local::now();
        let figure = standard_font.convert(date.format("%H:%M:%S").to_string().as_str());
        screen.clear();
        screen.write(figure.unwrap().to_string().as_str());
        sleep(Duration::new(1, 0));
    }
}
