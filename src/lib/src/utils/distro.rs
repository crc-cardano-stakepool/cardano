use crate::{async_command_pipe, install_packages, update, DEBIAN_PACKAGES, NON_DEBIAN_PACKAGES};
use anyhow::{anyhow, Result};
use sysinfo::{System, SystemExt, CpuExt, DiskExt};

pub fn get_sysinfo() {
    let mut sys = System::new_all();
    sys.refresh_all();

    log::info!("Getting system info");
    log::debug!("System name: {:?}", sys.name());

    log::info!("Getting disk info");
    for disk in sys.disks() {
        log::info!("Found disk: {disk:#?}");
        let disk_type = disk.type_();
        log::debug!("Disk type: {:?}", disk_type);
        let file_system = disk.file_system();
        log::debug!("Disk filesystem: {:?}", file_system);
        let mount_point = disk.mount_point();
        log::debug!("Disk mounted at: {:?}", mount_point);
        let total_space = disk.total_space();
        log::debug!("Total disk space: {:?} B", total_space);
        let available_space = disk.available_space();
        log::debug!("Total available disk space: {:?} B", available_space);
    }

    log::info!("Getting memory info");
    log::debug!("total memory    : {} KB", sys.total_memory());
    log::debug!("used memory     : {} KB", sys.used_memory());
    log::debug!("available memory: {} KB", sys.available_memory());
    log::debug!("total swap      : {} KB", sys.total_swap());
    log::debug!("used swap       : {} KB", sys.used_swap());

    log::info!("Getting CPU info");
    log::debug!("CPU info        : {:?}", sys.global_cpu_info().brand());
    log::debug!("Physical cores  : {:?}", sys.physical_core_count());
}

pub fn check_distro_result(distro: Result<String>) -> Result<String> {
    match distro {
        Ok(result) => Ok(result),
        Err(e) => Err(anyhow!("Failed checking distribution with error: {e}")),
    }
}

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
        Err(e) => Err(anyhow!("Failed checking distro with error: {e}")),
    }
}

pub async fn install_distro_packages(distro: &str) -> Result<()> {
    match distro {
        "ubuntu" | "debian" => {
            let package_manager = "apt";
            update(package_manager).await?;
            install_packages(package_manager, &DEBIAN_PACKAGES).await
        }
        "Fedora" | "Hat" | "CentOs" => {
            let package_manager = "yum";
            update(package_manager).await?;
            install_packages(package_manager, &NON_DEBIAN_PACKAGES).await
        }
        _ => Err(anyhow!("Unsupported distro: {distro}")),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_sysinfo() {
        get_sysinfo()
    }

    #[tokio::test]
    #[ignore]
    async fn test_install_distro_packages() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_distro() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_check_distro_result() {
        unimplemented!();
    }
}
