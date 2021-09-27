use crate::{async_command_pipe, check_distro_result};
use anyhow::{anyhow, Result};

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
        Err(e) => Err(anyhow!("{}", e)),
    }
}
