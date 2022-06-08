use crate::{async_command_pipe, check_distro_result};
use anyhow::{anyhow, Result};

// TODO: Use lib sysinfo for this
pub async fn check_distro() -> Result<String> {
    let cmd = format!("cat /etc/*ease | grep ID_LIKE | awk -F '=' {}", "'{print $2}'");
    let distro = async_command_pipe(&cmd).await;
    match distro {
        Ok(distro) => {
            if distro.is_empty() {
                let cmd = format!("cat /etc/*ease | grep ID | awk -F '=' {} | tail -n1", "'{print $2}'");
                let distro = async_command_pipe(&cmd).await;
                check_distro_result(distro)
            } else {
                check_distro_result(Ok(distro))
            }
        }
        Err(e) => Err(anyhow!("Failed checking distro with error: {}", e)),
    }
}

#[cfg(test)]
mod test {
    // use crate::check_distro;
    #[tokio::test]
    #[ignore]
    async fn test_check_distro() {
        unimplemented!();
    }
}