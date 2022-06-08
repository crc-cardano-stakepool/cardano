use crate::{async_command, print};
use anyhow::Result;

pub async fn change_dir(absolute_path: &str) -> Result<()> {
    let cmd = format!("cd {}", absolute_path);
    if async_command(&cmd).await.is_ok() {
        let msg = format!("Changed directory to {}", absolute_path);
        print("green", &msg)
    } else {
        let msg = format!("Failed changing directory to {}", absolute_path);
        print("red", &msg)
    }
}

#[cfg(test)]
mod test {
    // use crate::change_dir;
    #[tokio::test]
    #[ignore]
    async fn test_change_dir() {
        unimplemented!();
    }
}
