use super::types::*;
use console::Color;
use console::Emoji;
use console::Style;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::process::Stdio;
use tokio::process::Command;

pub struct Terminal;

impl Terminal {
    pub async fn async_command(color: &str, command: &str, emoji: Emoji<'_, '_>) -> TResult<()> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .output()
            .await;
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Terminal::print(color, &stdout, emoji).await.expect("Failed printing to terminal");
            Ok(())
        } else {
            panic!("Error executing command: {}", command);
        }
    }

    pub async fn print(color: &str, output: &str, emoji: Emoji<'_, '_>) -> TResult<()> {
        if let Ok(color) = Terminal::to_color(&color) {
            match color {
                Color::Cyan => {
                    let cyan = format!("{} {}", Style::new().cyan().apply_to(output), emoji);
                    Terminal::write(&cyan).await.expect("Failed printing to terminal");
                }
                Color::Blue => {
                    let blue = format!("{} {}", Style::new().blue().apply_to(output), emoji);
                    Terminal::write(&blue).await.expect("Failed printing to terminal");
                }
                Color::Black => {
                    let black = format!("{} {}", Style::new().black().apply_to(output), emoji);
                    Terminal::write(&black).await.expect("Failed printing to terminal");
                }
                Color::Red => {
                    let red = format!("{} {}", Style::new().red().apply_to(output), emoji);
                    Terminal::write(&red).await.expect("Failed printing to terminal");
                }
                Color::Green => {
                    let green = format!("{} {}", Style::new().green().apply_to(output), emoji);
                    Terminal::write(&green).await.expect("Failed printing to terminal");
                }
                Color::Yellow => {
                    let yellow = format!("{} {}", Style::new().yellow().apply_to(output), emoji);
                    Terminal::write(&yellow).await.expect("Failed printing to terminal");
                }
                Color::Magenta => {
                    let magenta = format!("{} {}", Style::new().magenta().apply_to(output), emoji);

                    Terminal::write(&magenta).await.expect("Failed printing to terminal");
                }
                Color::White => {
                    let white = format!("{} {}", Style::new().white().apply_to(output), emoji);
                    Terminal::write(&white).await.expect("Failed printing to terminal");
                }
                _ => {
                    let white = format!("{} {}", Style::new().white().apply_to(output), emoji);
                    Terminal::write(&white).await.expect("Failed printing to terminal");
                }
            };
        }
        Ok(())
    }

    fn to_color(color: &str) -> TResult<Color> {
        match color {
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "white" => Ok(Color::White),
            _ => Ok(Color::White),
        }
    }

    async fn write(output: &str) -> TResult<()> {
        if let Ok(()) = Term::stdout().write_line(output) {
        } else {
            panic!("Error writing to terminal");
        }
        Ok(())
    }

    pub fn proceed() -> TResult<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you wish to continue?")
            .interact()?;
        Ok(true)
    }
}
