use crate::{
    match_network, network_to_string, Executer, FileSystem, Node,
    CSNAPSHOT_BASE_URL, CSNAPSHOT_DATA_URL, CSNAPSHOT_DOWNLOAD_URL,
};
use anyhow::Result;
use cardano_multiplatform_lib::NetworkIdKind;

impl Node {
    pub fn download_snapshot(network: NetworkIdKind) -> Result<()> {
        let network = network_to_string(network);
        log::info!("Downloading ledger snapshot from {CSNAPSHOT_BASE_URL}");
        let mut path = Self::get_config(match_network(&network))?.unwrap();
        path.pop();
        let db_path = FileSystem::path_to_string(&path)?;
        let mut download_path = Self::get_db(match_network(&network))?.unwrap();
        let name = format!("{network}-snapshot.tar.lz4");
        download_path.push(name);
        let download_path = FileSystem::path_to_string(&download_path)?;
        let cmd = format!(
            "wget {CSNAPSHOT_DOWNLOAD_URL}/{network}/$(curl \
             -s {CSNAPSHOT_DATA_URL}/{network}-db-snapshot.json | \
             jq -r '.[]'.file_name) --output-document {download_path} && \
             lz4 -dvc --no-sparse {download_path} | tar x -C {db_path} \
             "
        );
        Executer::exec(&cmd)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_download_snapshot() -> Result<()> {
        Node::check_config_files(NetworkIdKind::Testnet)?;
        Node::download_snapshot(NetworkIdKind::Testnet)?;
        Node::check_config_files(NetworkIdKind::Mainnet)?;
        Node::download_snapshot(NetworkIdKind::Mainnet)?;
        Ok(())
    }
}
