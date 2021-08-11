use console::Color;
use console::Style;
use console::Term;

pub fn write_color(s: &str, c: Color) {
    match c {
        Color::Cyan => {
            let cyan = format!("{}", Style::new().cyan().apply_to(s));
            write(&cyan)
        }
        Color::Blue => {
            let blue = format!("{}", Style::new().blue().apply_to(s));
            write(&blue)
        }
        Color::Black => {
            let black = format!("{}", Style::new().black().apply_to(s));
            write(&black)
        }
        Color::Red => {
            let red = format!("{}", Style::new().red().apply_to(s));
            write(&red)
        }
        Color::Green => {
            let green = format!("{}", Style::new().green().apply_to(s));
            write(&green)
        }
        Color::Yellow => {
            let yellow = format!("{}", Style::new().yellow().apply_to(s));
            write(&yellow)
        }
        Color::Magenta => {
            let magenta = format!("{}", Style::new().magenta().apply_to(s));
            write(&magenta)
        }
        Color::White => {
            let white = format!("{}", Style::new().white().apply_to(s));
            write(&white)
        }
        _ => {
            let white = format!("{}", Style::new().white().apply_to(s));
            write(&white)
        }
    };
}

pub fn write(s: &str) {
    let term = Term::stdout();
    match term.write_line(s) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}
