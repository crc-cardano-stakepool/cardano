use crate::{
    async_command, get_config, get_db, path_to_string, CARDANO_BLOCKCHAIN_CSNAPSHOT_BASE_URL, CARDANO_BLOCKCHAIN_CSNAPSHOT_DATA_URL,
    CARDANO_BLOCKCHAIN_CSNAPSHOT_DOWNLOAD_URL,
};
use anyhow::Result;

pub async fn download_snapshot(network: &str) -> Result<()> {
    log::info!("Downloading ledger snapshot from {CARDANO_BLOCKCHAIN_CSNAPSHOT_BASE_URL}");
    let mut path = get_config(network)?.unwrap();
    path.pop();
    let db_path = path_to_string(&path)?;
    let mut download_path = get_db(network)?.unwrap();
    let name = format!("{network}-snapshot.tar.lz4");
    download_path.push(name);
    let download_path = path_to_string(&download_path)?;
    let cmd = format!(
        "wget {CARDANO_BLOCKCHAIN_CSNAPSHOT_DOWNLOAD_URL}/{network}/$(curl \
         -s {CARDANO_BLOCKCHAIN_CSNAPSHOT_DATA_URL}/{network}-db-snapshot.json | \
         jq -r '.[]'.file_name) --output-document {download_path} && \
         lz4 -dvc --no-sparse {download_path} | tar x -C {db_path} \
         "
    );
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::check_config_files;

    #[tokio::test]
    #[ignore]
    async fn test_download_snapshot() -> Result<()> {
        let network = "testnet";
        check_config_files(network).await?;
        download_snapshot(network).await?;
        let network = "mainnet";
        check_config_files(network).await?;
        download_snapshot(network).await?;
        Ok(())
    }
}
