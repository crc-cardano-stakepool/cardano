use dialoguer::{theme::ColorfulTheme, Confirm};
use std::io::Result;

pub struct Dialog;

impl Dialog {
    pub fn proceed(prompt: &str) -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(String::from(prompt))
            .interact()
    }
}
