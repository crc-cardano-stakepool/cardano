use crate::{
    async_command, async_user_command, check_cabal, check_env, check_ghc, export_shell_variables, file_exists,
    get_component_path, get_project_file, print, update_project_file,
};
use anyhow::Result;

pub async fn configure_build(component: &str, ghc_version: &str) -> Result<()> {
    print("", "Checking build dependencies")?;
    check_ghc().await?;
    check_cabal().await?;
    let path = get_component_path(component).await?;
    let cabal = check_env("CABAL_BIN")?;
    let ghc = check_env("GHC_BIN")?;
    let project_file = get_project_file(component).await?;
    if file_exists(&project_file) {
        let cmd = format!("rm {}", project_file);
        async_command(&cmd).await?;
    }
    print("", "Updating Cabal")?;
    let cmd = format!("cd {} && {} update", path, cabal);
    async_user_command(&cmd).await?;
    print("green", "Updated Cabal successfully")?;
    export_shell_variables().await?;
    let cmd = format!(
        "cd {} && {} configure --with-compiler={}-{}",
        path, cabal, ghc, ghc_version
    );
    async_user_command(&cmd).await?;
    let msg = format!("Configured build of {} successfully", component);
    print("green", &msg)?;
    update_project_file(component, &project_file).await?;
    let msg = format!("Building {}", component);
    print("", &msg)?;
    let cmd = format!("cd {} && {} build all", path, cabal);
    async_user_command(&cmd).await?;
    Ok(())
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
