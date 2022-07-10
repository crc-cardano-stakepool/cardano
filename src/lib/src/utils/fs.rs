use crate::{
    CardanoComponent, Component, Environment, Executer, Git, Settings,
    DIRECTORIES,
};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::{
    fs::create_dir_all,
    io::Write,
    path::{Path, PathBuf},
};
pub struct FileSystem;
impl FileSystem {
    pub fn setup_work_dir() -> Result<()> {
        log::debug!("Setting up working directory");
        for key in DIRECTORIES {
            let key = format!("{key}_dir");
            let directory = Settings::read(&key)?;
            Self::check_dir(&directory)?;
            let key = key.to_case(Case::UpperSnake);
            Environment::set_env(&key, &directory);
        }
        Ok(())
    }

    pub fn check_dir<P: AsRef<Path>>(absolute_path: P) -> Result<()> {
        let path = Self::path_to_string(absolute_path.as_ref())?;
        log::trace!("Checking {path}");
        if !absolute_path.as_ref().is_dir() {
            log::debug!("{path} is not a directory");
            return Self::create_dir(absolute_path);
        }
        log::trace!("The path {path} exists");
        Ok(())
    }

    pub fn check_work_dir() -> Result<impl AsRef<Path>> {
        log::debug!("Checking the working directory");
        Settings::read("work_dir").map(PathBuf::from)
    }

    pub fn copy_binary(component: Component) -> Result<()> {
        let component = CardanoComponent::component_to_string(component);
        log::debug!("Copying the built binaries of {component}");
        let install_dir = Environment::check_env("INSTALL_DIR")?;
        let component_enum = CardanoComponent::match_component(&component);
        match component_enum {
            Component::Node => Self::copy_node_binaries(&install_dir),
            Component::Cli => Self::copy_node_binaries(&install_dir),
            Component::Wallet | Component::Bech32 => {
                let install_dir = Self::path_to_string(install_dir.as_ref())?;
                log::info!(
                    "Installing the built {component} binary to {install_dir}"
                );
                let path = Git::set_component_dir(component_enum)?;
                let cmd = format!(
                    "cd {path} && \
                    cabal install {component} \
                    --install-method=copy \
                    --overwrite-policy=always \
                    --installdir={install_dir}"
                );
                Executer::exec(&cmd)?;
                Ok(())
            }
            Component::Address => Self::copy_address_binary(&install_dir),
        }
    }

    pub fn copy_address_binary<P: AsRef<Path>>(install_dir: P) -> Result<()> {
        let install_dir = Self::path_to_string(install_dir.as_ref())?;
        log::info!(
            "Installing the built cardano-address binary to {install_dir}"
        );
        let path = Git::set_component_dir(Component::Address)?;
        let cmd = format!(
            "cd {path} && \
            cabal install cardano-addresses-cli \
            --install-method=copy \
            --overwrite-policy=always \
            --installdir={install_dir}"
        );
        Executer::exec(&cmd)
    }

    pub fn copy_node_binaries<P: AsRef<Path>>(install_dir: P) -> Result<()> {
        let install_dir =
            Self::absolute_ref_path_to_string(install_dir.as_ref())?;
        let mut path = CardanoComponent::get_component_path(Component::Node)?;
        let parsed_path = Self::absolute_ref_path_to_string(&path)?;
        let bin_path = format!("{parsed_path}/scripts/bin-path.sh");
        path.push("scripts");
        path.push("bin-path.sh");
        let components = ["cardano-node", "cardano-cli"];
        for component in components {
            let cmd = format!("cd {parsed_path} && cp -p \"$({bin_path} {component})\" {install_dir}");
            let path = format!("{install_dir}/{component}");
            if component.eq("cardano-node") {
                Environment::set_env("CARDANO_NODE_BIN", &path);
            } else {
                Environment::set_env("CARDANO_CLI_BIN", &path);
            }
            log::info!("Copying built {component} binary to {path}");
            Executer::exec(&cmd)?;
        }
        Ok(())
    }
    pub fn create_dir<P: AsRef<Path>>(absolute_path: P) -> Result<()> {
        create_dir_all(&absolute_path)?;
        Self::absolute_ref_path_to_string(&absolute_path).map(|path| {
            log::info!("Created directory: {path}");
        })
    }

    pub fn path_to_string(path: &Path) -> Result<String> {
        log::trace!("Parsing the absolute path to a string");
        if let Some(path) = path.to_str() {
            return Ok(path.to_string());
        }
        Err(anyhow!("Failed to parse path to string"))
    }

    pub fn absolute_ref_path_to_string<P: AsRef<Path>>(
        absolute_path: P,
    ) -> Result<String> {
        log::trace!("Parsing the path to string if the path is absolute");
        let path = absolute_path.as_ref();
        let parsed = Self::path_to_string(path)?;
        if !path.exists() {
            log::error!("The path {parsed} does not exist");
            return Err(anyhow!("The path {parsed} does not exist"));
        }
        if path.is_absolute() {
            return Self::path_to_string(path);
        }
        Err(anyhow!("The path {parsed} is not absolute"))
    }

    pub fn get_bin_path(bin: &str) -> Result<PathBuf> {
        log::debug!("Getting the path of the binary {bin}");
        if let Some(mut path) = dirs::executable_dir() {
            path.push(bin);
            if !path.exists() {
                return Err(anyhow!("The {bin} binary was not found"));
            }
            let parsed = Self::absolute_ref_path_to_string(&path)?;
            log::debug!("The path to the {bin} binary: {parsed}");
            return Ok(path);
        };
        Err(anyhow!(
            "XDG_BIN_HOME is not set, failed to check if {bin} is installed"
        ))
    }

    pub fn check_project_file<P: AsRef<Path>>(project_file: P) -> Result<()> {
        log::debug!("Checking if the project file already exists");
        let file = project_file.as_ref();
        let path = Self::path_to_string(file)?;
        log::debug!("Project file: {path}");
        if !file.is_file() {
            log::debug!("Project file {path} is not a file");
            return Ok(());
        }
        let file_name = file.file_name().unwrap().to_str().unwrap();
        log::debug!("File name: {file_name}");
        if file_name.eq("cabal.project.local") {
            log::warn!("Project file already exists, removing it");
            let cmd = format!("rm {path}");
            return Executer::exec(&cmd);
        }
        Ok(())
    }

    pub fn update_project_file<P: AsRef<Path>>(path: P) -> Result<()> {
        let file_path = Self::absolute_ref_path_to_string(&path)?;
        if !path.as_ref().is_file() {
            return Err(anyhow!("The path {file_path} is not a file"));
        }
        let file_name = path.as_ref().file_name().unwrap().to_str().unwrap();
        log::debug!("File name: {file_name}");
        if !file_name.eq("cabal.project.local") {
            log::error!("Unexpected filename: {file_name}");
            return Err(anyhow!("Unexpected filename: {file_name}"));
        }
        log::info!("Updating the project file at {file_path}");
        let mut f = std::fs::File::options()
            .write(true)
            .append(true)
            .open(path)
            .map_err(|err| anyhow!("Failed to open {file_path}: {err}"))
            .unwrap();
        let value =
            "package cardano-crypto-praos\n  flags: -external-libsodium-vrf";
        writeln!(f, "{value}")
            .map_err(|err| {
                anyhow!("Failed to write {value} to {file_path}: {err}")
            })
            .unwrap();
        Ok(())
    }

    pub fn get_project_file(component: Component) -> Result<PathBuf> {
        let component = CardanoComponent::component_to_string(component);
        log::debug!(
            "Getting the project file of the {component} source reposity"
        );
        CardanoComponent::get_component_path(CardanoComponent::match_component(
            &component,
        ))
        .map(|mut path| {
            path.push("cabal.project.local");
            path
        })
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::io::{Read, Seek, SeekFrom};

    #[test]
    fn test_setup_work_dir() -> Result<()> {
        FileSystem::setup_work_dir()?;
        for key in DIRECTORIES {
            let key = format!("{key}_dir");
            let setting = Settings::read(&key)?;
            let key = key.to_case(Case::UpperSnake);
            let value = Environment::check_env(&key)?;
            assert_eq!(value, setting);
        }
        Ok(())
    }

    #[test]
    fn test_check_work_dir() -> Result<()> {
        let home = dirs::home_dir().unwrap();
        let home = home.to_str().unwrap();
        log::debug!("{home}");
        let work_dir = FileSystem::check_work_dir()?;
        let work_dir = work_dir.as_ref().to_str().unwrap();
        log::debug!("{work_dir}");
        let result = Environment::check_env("WORK_DIR")?;
        log::debug!("{result}");
        assert_eq!(work_dir, result);
        Ok(())
    }
    #[test]
    fn test_get_project_file() {
        let component = Component::Node;
        Git::set_component_dir(component).unwrap();
        let mut path = CardanoComponent::get_component_path(component).unwrap();
        path.push("cabal.project.local");
        let project_file = FileSystem::get_project_file(component).unwrap();
        assert_eq!(path, project_file)
    }

    #[test]
    fn test_check_project_file() {
        let file_name = "cabal.project.local";
        let dir = tempfile::Builder::new().tempdir().unwrap();
        let file_path = dir.path().join(file_name);
        let project_file_name =
            file_path.file_name().unwrap().to_str().unwrap();
        std::fs::File::create(&file_path).unwrap();
        assert_eq!(file_name, project_file_name);
        FileSystem::check_project_file(&file_path).unwrap();
        assert!(!file_path.exists());
    }

    #[test]
    fn test_update_project_file() {
        let project_file = tempfile::Builder::new()
            .tempdir()
            .unwrap()
            .path()
            .with_file_name("cabal.project.local");
        std::fs::File::create(&project_file).unwrap();
        FileSystem::update_project_file(&project_file).unwrap();
        let mut project_file = std::fs::File::open(project_file).unwrap();
        project_file.seek(SeekFrom::Start(0)).unwrap();
        let mut buf = String::new();
        project_file.read_to_string(&mut buf).unwrap();
        let expected = format!(
            "package cardano-crypto-praos\n  flags: -external-libsodium-vrf\n"
        );
        assert_eq!(expected, buf);
    }
}
