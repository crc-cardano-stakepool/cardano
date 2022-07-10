use crate::{Executer, SystemInfo};
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
    pub platform: Platform,
    pub distro: Distro,
    pub package_manager: PackageManager,
    pub packages: Vec<String>,
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
                log::error!("Consider fetching the latest binary directly");
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

    pub fn setup_packages(&self) -> Result<()> {
        self.update()?;
        self.check_packages()
    }

    fn check_platform() -> Platform {
        log::debug!("Checking current platform");
        let platform = Executer::capture("uname").unwrap();
        let platform = platform.trim();
        match platform {
            "linux" | "Linux" => Platform::Linux,
            _ => Platform::Unsupported {
                platform: platform.to_string(),
            },
        }
    }

    fn get_package_manager(&self) -> String {
        match self.package_manager {
            PackageManager::Apt => "apt".to_string(),
            PackageManager::Yum => "yum".to_string(),
        }
    }

    fn update(&self) -> Result<()> {
        let package_manager = self.get_package_manager();
        log::info!("Updating system with {package_manager}");
        let cmd = format!("sudo {package_manager} update -y");
        Executer::exec(&cmd)
    }

    fn check_packages(&self) -> Result<()> {
        log::debug!("Checking packages");
        for package in self.packages.iter() {
            self.check_package(package)?;
        }
        Ok(())
    }

    fn check_package(&self, package: &str) -> Result<()> {
        log::debug!("Checking if {package} is installed");
        match self.package_manager {
            PackageManager::Apt => self.apt_install(package),
            PackageManager::Yum => self.yum_install(package),
        }
    }

    fn apt_install(&self, package: &str) -> Result<()> {
        let cmd = format!("dpkg -s {}", package.trim());
        let piped_cmd = "grep installed";
        if let Ok(result) = Executer::pipe(&cmd, piped_cmd) {
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
        if Executer::success(&cmd)? {
            log::debug!("{package} is installed");
            return Ok(());
        }
        self.install_package(package)
    }

    fn install_package(&self, package: &str) -> Result<()> {
        let package_manager = self.get_package_manager();
        log::info!("Installing {package} with {package_manager}");
        let cmd = format!("sudo {package_manager} install {package} -y");
        if let Err(err) = Executer::exec(&cmd) {
            return Err(anyhow!("Failed installing {package}: {err}"));
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
