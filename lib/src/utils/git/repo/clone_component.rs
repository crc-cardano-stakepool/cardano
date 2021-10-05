use crate::{check_env, check_repo, checkout_latest_release, set_env, URLS};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};

pub async fn clone_component(component: &str) -> Result<()> {
    if let Some(url) = URLS.get(component) {
        let work_dir = check_env("WORK_DIR")?;
        let cardano_component_dir = format!("{}/{}", work_dir, component);
        let env_name = format!("{}-dir", component);
        let converted = env_name.to_case(Case::UpperSnake);
        set_env(&converted, &cardano_component_dir);
        check_repo(url, &cardano_component_dir, "cardano-node").await?;
        checkout_latest_release(component).await
    } else {
        Err(anyhow!("Failed cloning {} repository", component))
    }
}

#[cfg(test)]
mod test {
    // use crate::clone_cardano_repo;
    #[tokio::test]
    #[ignore]
    async fn test_clone_cardano_repo() {
        unimplemented!();
    }
}
