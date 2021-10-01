use super::to_color;
use anyhow::Result;
use console::{Color, Emoji, Style, Term};

pub fn print_emoji(
    color: &str,
    output: &str,
    emoji: Emoji<'_, '_>,
) -> Result<String> {
    match to_color(color) {
        Color::Cyan => {
            let cyan = format!(
                "{} {}",
                Style::new().cyan().bold().apply_to(output),
                emoji
            );
            Term::stdout().write_line(&cyan)?;
            Ok(cyan)
        }
        Color::Blue => {
            let blue = format!(
                "{} {}",
                Style::new().blue().bold().apply_to(output),
                emoji
            );
            Term::stdout().write_line(&blue)?;
            Ok(blue)
        }
        Color::Black => {
            let black = format!(
                "{} {}",
                Style::new().black().bold().apply_to(output),
                emoji
            );
            Term::stdout().write_line(&black)?;
            Ok(black)
        }
        Color::Red => {
            let red = format!(
                "{} {}",
                Style::new().red().bold().apply_to(output),
                emoji
            );
            Term::stdout().write_line(&red)?;
            Ok(red)
        }
        Color::Green => {
            let green = format!(
                "{} {}",
                Style::new().green().bold().apply_to(output),
                emoji
            );
            Term::stdout().write_line(&green)?;
            Ok(green)
        }
        Color::Yellow => {
            let yellow = format!(
                "{} {}",
                Style::new().yellow().bold().apply_to(output),
                emoji
            );
            Term::stdout().write_line(&yellow)?;
            Ok(yellow)
        }
        Color::Magenta => {
            let magenta = format!(
                "{} {}",
                Style::new().magenta().bold().apply_to(output),
                emoji
            );
            Term::stdout().write_line(&magenta)?;
            Ok(magenta)
        }
        Color::White => {
            let white = format!(
                "{} {}",
                Style::new().white().bold().apply_to(output),
                emoji
            );
            Term::stdout().write_line(&white)?;
            Ok(white)
        }
        _ => {
            let white = format!(
                "{} {}",
                Style::new().white().bold().apply_to(output),
                emoji
            );
            Term::stdout().write_line(&white)?;
            Ok(white)
        }
    }
}

#[cfg(test)]
mod test {
    // use crate::print_emoji;
    #[tokio::test]
    #[ignore]
    async fn test_print_emoji() {
        unimplemented!();
    }
}
