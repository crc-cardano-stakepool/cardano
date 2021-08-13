use crate::cli::types::TResult;
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn proceed() -> TResult<()> {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you wish to continue?")
        .interact()?;
    Ok(())
}
