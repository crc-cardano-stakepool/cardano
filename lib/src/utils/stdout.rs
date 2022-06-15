use anyhow::Result;
use console::{Color, Emoji, Style, Term};

pub fn print_emoji(color: &str, output: &str, emoji: Emoji<'_, '_>) -> Result<()> {
    match to_color(color) {
        Color::Cyan => {
            let cyan = format!("{} {}", Style::new().cyan().bold().apply_to(output), emoji);
            Term::buffered_stdout().write_line(&cyan)?;
        }
        Color::Blue => {
            let blue = format!("{} {}", Style::new().blue().bold().apply_to(output), emoji);
            Term::buffered_stdout().write_line(&blue)?;
        }
        Color::Black => {
            let black = format!("{} {}", Style::new().black().bold().apply_to(output), emoji);
            Term::buffered_stdout().write_line(&black)?;
        }
        Color::Red => {
            let red = format!("{} {}", Style::new().red().bold().apply_to(output), emoji);
            Term::buffered_stdout().write_line(&red)?;
        }
        Color::Green => {
            let green = format!("{} {}", Style::new().green().bold().apply_to(output), emoji);
            Term::buffered_stdout().write_line(&green)?;
        }
        Color::Yellow => {
            let yellow = format!("{} {}", Style::new().yellow().bold().apply_to(output), emoji);
            Term::buffered_stdout().write_line(&yellow)?;
        }
        Color::Magenta => {
            let magenta = format!("{} {}", Style::new().magenta().bold().apply_to(output), emoji);
            Term::buffered_stdout().write_line(&magenta)?;
        }
        Color::White => {
            let white = format!("{} {}", Style::new().white().bold().apply_to(output), emoji);
            Term::buffered_stdout().write_line(&white)?;
        }
        _ => {
            let white = format!("{} {}", Style::new().white().bold().apply_to(output), emoji);
            Term::buffered_stdout().write_line(&white)?;
        }
    }
    Ok(())
}

pub fn print(color: &str, output: &str) -> Result<()> {
    match to_color(color) {
        Color::Cyan => {
            let cyan = format!("{}", Style::new().cyan().bold().apply_to(output));
            Term::buffered_stdout().write_line(&cyan)?;
        }
        Color::Blue => {
            let blue = format!("{}", Style::new().blue().bold().apply_to(output));
            Term::buffered_stdout().write_line(&blue)?;
        }
        Color::Black => {
            let black = format!("{}", Style::new().black().bold().apply_to(output));
            Term::buffered_stdout().write_line(&black)?;
        }
        Color::Red => {
            let red = format!("{}", Style::new().red().bold().apply_to(output));
            Term::buffered_stdout().write_line(&red)?;
        }
        Color::Green => {
            let green = format!("{}", Style::new().green().bold().apply_to(output));
            Term::buffered_stdout().write_line(&green)?;
        }
        Color::Yellow => {
            let yellow = format!("{}", Style::new().yellow().bold().apply_to(output));
            Term::buffered_stdout().write_line(&yellow)?;
        }
        Color::Magenta => {
            let magenta = format!("{}", Style::new().magenta().bold().apply_to(output));
            Term::buffered_stdout().write_line(&magenta)?;
        }
        Color::White => {
            let white = format!("{}", Style::new().white().bold().apply_to(output));
            Term::buffered_stdout().write_line(&white)?;
        }
        _ => {
            let white = format!("{}", Style::new().white().bold().apply_to(output));
            Term::buffered_stdout().write_line(&white)?;
        }
    };
    Ok(())
}

pub fn to_color(color: &str) -> Color {
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

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_to_color() {
        unimplemented!();
    }

    #[test]
    fn test_print() -> Result<()> {
        print("", "test")
    }

    #[tokio::test]
    #[ignore]
    async fn test_print_emoji() {
        unimplemented!();
    }
}
