use dialoguer::{theme::ColorfulTheme, Confirm};
use std::io::Result;

pub fn proceed(prompt: &str) -> Result<bool> {
    let color_theme = &ColorfulTheme::default();
    let dialog = String::from(prompt);
    Confirm::with_theme(color_theme).with_prompt(dialog).interact()
}

#[cfg(test)]
mod test {
    // use crate::proceed;
    #[tokio::test]
    #[ignore]
    async fn test_proceed() {
        unimplemented!();
    }
}
