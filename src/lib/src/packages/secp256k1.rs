use crate::{Environment, Executer, Git, ShellConfig, SECP256K1_URL};
use anyhow::Result;
use std::path::Path;

pub struct Secp256k1;

impl Secp256k1 {
    pub fn check() -> Result<()> {
        log::debug!("Checking secp256k1");
        let pc = Path::new("/usr/local/lib/pkgconfig/libsecp256k1.pc");
        let so = Path::new("/usr/local/lib/libsecp256k1.so");
        let so_0 = Path::new("/usr/local/lib/libsecp256k1.so.0");
        let so_0_0_0 = Path::new("/usr/local/lib/libsecp256k1.so.0.0.0");
        let la = Path::new("/usr/local/lib/libsecp256k1.la");
        let a = Path::new("/usr/local/lib/libsecp256k1.a");
        if !(pc.is_file()
            && so.is_file()
            && la.is_file()
            && so_0.is_file()
            && so_0_0_0.is_file()
            && a.is_file())
        {
            log::warn!("secp256k1 is not installed");
            Self::install()?;
        }
        Ok(())
    }

    pub fn install() -> Result<()> {
        log::info!("Installing secp256k1");
        let secp256k1_path = Environment::check_env("SECP_256_K_1_DIR")?;
        Git::check_repo(SECP256K1_URL, &secp256k1_path)?;
        let checkout = "git checkout ac83be33";
        let configure =
            "./configure --enable-module-schnorrsig --enable-experimental";
        let cmd = format!("cd {secp256k1_path} && {checkout} && ./autogen.sh && {configure} && make");
        Executer::exec(&cmd)?;
        let cmd = format!("cd {secp256k1_path} && sudo make install");
        Executer::exec(&cmd)?;
        Executer::exec("sudo ldconfig")?;
        ShellConfig::source_shell()
    }
}

#[cfg(test)]
mod test {
    // use super::*;
}
