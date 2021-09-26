use super::to_color;
use anyhow::Result;
use console::{Color, Style, Term};

pub fn print(color: &str, output: &str) -> Result<()> {
    match to_color(color) {
        Color::Cyan => {
            let cyan = format!("{}", Style::new().cyan().apply_to(output));
            Term::stdout().write_line(&cyan)?;
        }
        Color::Blue => {
            let blue = format!("{}", Style::new().blue().apply_to(output));
            Term::stdout().write_line(&blue)?;
        }
        Color::Black => {
            let black = format!("{}", Style::new().black().apply_to(output));
            Term::stdout().write_line(&black)?;
        }
        Color::Red => {
            let red = format!("{}", Style::new().red().apply_to(output));
            Term::stdout().write_line(&red)?;
        }
        Color::Green => {
            let green = format!("{}", Style::new().green().apply_to(output));
            Term::stdout().write_line(&green)?;
        }
        Color::Yellow => {
            let yellow = format!("{}", Style::new().yellow().apply_to(output));
            Term::stdout().write_line(&yellow)?;
        }
        Color::Magenta => {
            let magenta = format!("{}", Style::new().magenta().apply_to(output));
            Term::stdout().write_line(&magenta)?;
        }
        Color::White => {
            let white = format!("{}", Style::new().white().apply_to(output));
            Term::stdout().write_line(&white)?;
        }
        _ => {
            let white = format!("{}", Style::new().white().apply_to(output));
            Term::stdout().write_line(&white)?;
        }
    };
    Ok(())
}
