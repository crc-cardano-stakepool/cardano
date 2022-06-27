use anyhow::Result;

#[derive(Debug)]
pub enum UpdateCommand {}

impl UpdateCommand {
    pub async fn update() -> Result<()> {
        println!("Updating the CLI");
        Ok(())
    }
}
