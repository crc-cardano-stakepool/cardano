use crate::{Environment, Executer, Git, ShellConfig, LIBSODIUM_URL};
use anyhow::Result;
use std::path::Path;

pub struct Libsodium;

impl Libsodium {
    pub fn check() -> Result<()> {
        log::debug!("Checking if libsodium is installed");
        let pc = Path::new("/usr/local/lib/pkgconfig/libsodium.pc");
        let so = Path::new("/usr/local/lib/libsodium.so");
        let so_23 = Path::new("/usr/local/lib/libsodium.so.23");
        let so_23_3_0 = Path::new("/usr/local/lib/libsodium.so.23.3.0");
        let la = Path::new("/usr/local/lib/libsodium.la");
        let a = Path::new("/usr/local/lib/libsodium.a");
        if !(pc.is_file()
            && so.is_file()
            && la.is_file()
            && so_23_3_0.is_file()
            && so_23.is_file()
            && a.is_file())
        {
            log::warn!("Libsodium is not installed");
            return Self::install();
        }
        Ok(())
    }

    pub fn install() -> Result<()> {
        log::info!("Installing libsodium");
        let libsodium_path = Environment::check_env("LIBSODIUM_DIR")?;
        Git::check_repo(LIBSODIUM_URL, &libsodium_path)?;
        let cmd = format!("cd {libsodium_path} && git checkout 66f017f1 && ./autogen.sh && ./configure && make");
        Executer::exec(&cmd)?;
        let cmd = format!("cd {libsodium_path} && sudo make install");
        Executer::exec(&cmd)?;
        ShellConfig::source_shell()
    }
}

#[cfg(test)]
mod test {
    // use super::*;
}
