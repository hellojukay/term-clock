use chrono::Local;
use figlet_rs::FIGfont;
use std::thread::sleep;
use std::time::Duration;
use console::Term;
use std::io;

fn main() ->io::Result<()>{
    let term = Term::stdout();
    term.clear_screen()?;
    term.hide_cursor()?;
    loop {
        let date = Local::now();
        let standard_font = FIGfont::standand().unwrap();
        let figure = standard_font.convert(date.format("%H:%M:%S").to_string().as_str());
        assert!(figure.is_some());
        println!("{}", figure.unwrap());

        // we sleep for 2 seconds
        sleep(Duration::new(1, 0));
        term.clear_screen()?;
        }
}
