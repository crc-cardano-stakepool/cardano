use crate::check_env;
use anyhow::Result;
use convert_case::{Case, Casing};

pub async fn get_component_path(component: &str) -> Result<String> {
    let env = format!("{}-dir", component);
    let converted = env.to_case(Case::UpperSnake);
    let path = check_env(&converted)?;
    Ok(path)
}

#[cfg(test)]
mod test {
    // use crate::get_component_path;
    #[tokio::test]
    #[ignore]
    async fn test_get_component_path() {
        unimplemented!();
    }
}
