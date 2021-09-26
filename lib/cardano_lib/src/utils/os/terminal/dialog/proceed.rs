use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn proceed(prompt: &str) -> Result<bool> {
    let color_theme = &ColorfulTheme::default();
    let dialog = String::from(prompt);
    if Confirm::with_theme(color_theme).with_prompt(dialog).interact()? {
        Ok(true)
    } else {
        Ok(false)
    }
}
