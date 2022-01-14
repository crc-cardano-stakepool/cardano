use crate::{
    async_command, async_user_command, check_env, check_user, clone_component, configure_build, file_exists,
    get_component_path, get_ghc_version, get_project_file, print, process_success_inherit, update_project_file,
};
use anyhow::{anyhow, Result};

pub async fn build_component(component: &str) -> Result<()> {
    clone_component(component).await?;
    let ghc_version = get_ghc_version().await?;
    let cabal = check_env("CABAL_BIN")?;
    let project_file = get_project_file(component).await?;
    let path = get_component_path(component).await?;
    update_cabal(&path, &cabal).await?;
    check_project_file(&project_file).await?;
    configure_build(component, &ghc_version, &path, &cabal).await?;
    update_project_file(component, &project_file).await?;
    build(component, &path, &cabal).await
}

async fn update_cabal(path: &str, cabal_path: &str) -> Result<()> {
    let cmd = format!("cd {} && {} update", path, cabal_path);
    print("", "Updating Cabal")?;
    async_user_command(&cmd).await?;
    print("green", "Updated Cabal successfully")
}

async fn check_project_file(project_file: &str) -> Result<()> {
    if file_exists(project_file) {
        let cmd = format!("rm {}", project_file);
        async_command(&cmd).await?;
        print("", "Removed project file")
    } else {
        Ok(())
    }
}

async fn build(component: &str, path: &str, cabal: &str) -> Result<()> {
    let user = check_user().await?;
    let cmd = format!("cd {} && {} build all", path, cabal);
    let cmd = format!("su - {} -c \"eval {}\"", user, cmd);
    let msg = format!("Building {}", component);
    print("", &msg)?;
    if process_success_inherit(&cmd).await? {
        let msg = format!("Successfully built {}", component);
        print("green", &msg)
    } else {
        Err(anyhow!("Failed building {}", component))
    }
}

#[cfg(test)]
mod test {
    // use crate::configure_build;
    #[tokio::test]
    #[ignore]
    async fn test_configure_build() {
        unimplemented!();
    }
}
