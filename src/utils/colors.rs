use console::Color;
use console::Emoji;
use console::Style;
use console::Term;

pub fn color(color: &str) -> Color {
    match color {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        _ => Color::White,
    }
}

pub fn write_color(s: &str, c: Color, e: Emoji) {
    match c {
        Color::Cyan => {
            let cyan = format!("{} {}", Style::new().cyan().apply_to(s), e);
            write(&cyan)
        }
        Color::Blue => {
            let blue = format!("{} {}", Style::new().blue().apply_to(s), e);
            write(&blue)
        }
        Color::Black => {
            let black = format!("{} {}", Style::new().black().apply_to(s), e);
            write(&black)
        }
        Color::Red => {
            let red = format!("{} {}", Style::new().red().apply_to(s), e);
            write(&red)
        }
        Color::Green => {
            let green = format!("{} {}", Style::new().green().apply_to(s), e);
            write(&green)
        }
        Color::Yellow => {
            let yellow = format!("{} {}", Style::new().yellow().apply_to(s), e);
            write(&yellow)
        }
        Color::Magenta => {
            let magenta = format!("{} {}", Style::new().magenta().apply_to(s), e);
            write(&magenta)
        }
        Color::White => {
            let white = format!("{} {}", Style::new().white().apply_to(s), e);
            write(&white)
        }
        _ => {
            let white = format!("{} {}", Style::new().white().apply_to(s), e);
            write(&white)
        }
    };
}

pub fn write(s: &str) {
    Term::stdout()
        .write_line(s)
        .expect("Failed printing to console")
}
