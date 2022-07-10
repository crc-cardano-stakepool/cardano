use crate::{
    async_command, async_command_pipe, drop_privileges, pipe, process_success,
    SystemInfo,
};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Platform {
    Linux,
    Unsupported { platform: String },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Distro {
    Ubuntu,
    Debian,
    Mint,
    CentOs,
    RedHat,
    Fedora,
    Unsupported { distro: String },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PackageManager {
    Apt,
    Yum,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PlatformInfo {
    platform: Platform,
    distro: Distro,
    package_manager: PackageManager,
    packages: Vec<String>,
}

impl Default for PlatformInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformInfo {
    pub fn new() -> Self {
        let platform = Self::check_platform();
        let distro = SystemInfo::get_sysinfo();
        let (package_manager, packages) = match distro {
            Distro::Ubuntu | Distro::Debian | Distro::Mint => (
                PackageManager::Apt,
                vec![
                    "curl",
                    "automake",
                    "build-essential",
                    "pkg-config",
                    "libffi-dev",
                    "libgmp-dev",
                    "libssl-dev",
                    "libtinfo-dev",
                    "libsystemd-dev",
                    "zlib1g-dev",
                    "liblz4-tool",
                    "libsodium-dev",
                    "tidy",
                    "make",
                    "g++",
                    "tmux",
                    "git",
                    "jq",
                    "wget",
                    "libncursesw5",
                    "libtool",
                    "autoconf",
                ]
                .iter_mut()
                .map(|package| package.to_string())
                .collect(),
            ),
            Distro::RedHat | Distro::CentOs | Distro::Fedora => (
                PackageManager::Yum,
                vec![
                    "curl",
                    "git",
                    "gcc",
                    "gcc-c++",
                    "tmux",
                    "gmp-devel",
                    "make",
                    "tar",
                    "xz",
                    "wget",
                    "zlib-devel",
                    "tidy",
                    "libtool",
                    "autoconf",
                    "systemd-devel",
                    "ncurses-devel",
                    "ncurses-compat-libs",
                ]
                .iter_mut()
                .map(|package| package.to_string())
                .collect(),
            ),
            Distro::Unsupported { distro } => {
                log::error!("Unsupported distro: {distro}");
                log::error!(
                    "Consider fetching the latest precompiled binary directly"
                );
                panic!("Unsupported distro: {distro}")
            }
        };
        Self {
            platform,
            distro,
            package_manager,
            packages,
        }
    }
    fn check_platform() -> Platform {
        log::debug!("Checking current platform");
        let platform = async_command_pipe("uname").unwrap();
        let platform = platform.trim();
        match platform {
            "linux" | "Linux" => Platform::Linux,
            _ => Platform::Unsupported {
                platform: platform.to_string(),
            },
        }
    }
    fn get_package_manager(&self) -> String {
        let pm = match self.package_manager {
            PackageManager::Apt => "apt",
            PackageManager::Yum => "yum",
        };
        String::from(pm)
    }
    pub fn setup_packages(&self) -> Result<()> {
        self.update()?;
        self.check_packages()?;
        Ok(())
    }
    fn update(&self) -> Result<()> {
        let package_manager = self.get_package_manager();
        log::info!("Updating system with {package_manager}");
        let cmd = format!("sudo {package_manager} update -y");
        async_command(&cmd)?;
        Ok(())
    }
    fn check_packages(&self) -> Result<()> {
        log::debug!("Checking packages");
        for package in self.packages.iter() {
            self.check_package(&self.package_manager, package)?;
        }
        drop_privileges()
    }
    fn check_package(
        &self,
        package_manager: &PackageManager,
        package: &str,
    ) -> Result<()> {
        log::debug!("Checking if {package} is installed");
        match package_manager {
            PackageManager::Apt => self.apt_install(package),
            PackageManager::Yum => self.yum_install(package),
        }
    }
    fn apt_install(&self, package: &str) -> Result<()> {
        let cmd = format!("dpkg -s {}", package.trim());
        let piped_cmd = "grep installed";
        if let Ok(result) = pipe(&cmd, piped_cmd) {
            if result.trim().is_empty() {
                return self.install_package(package);
            }
            log::debug!("{package} is installed");
            return Ok(());
        }
        Err(anyhow!("Failed installing {package}"))
    }
    fn yum_install(&self, package: &str) -> Result<()> {
        let cmd = format!("rpm -q {package}");
        if !process_success(&cmd)? {
            return self.install_package(package);
        }
        log::debug!("{package} is installed");
        Ok(())
    }
    fn install_package(&self, package: &str) -> Result<()> {
        let package_manager = self.get_package_manager();
        log::info!("Installing {package} with {package_manager}");
        let cmd = format!("sudo {package_manager} install {package} -y");
        if let Err(err) = async_command(&cmd) {
            return Err(anyhow!(
                "Failed installing {package} with error: {err}"
            ));
        }
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_platform_info() {
        let platform = PlatformInfo::default();
        log::debug!("{platform:#?}");
    }
}
