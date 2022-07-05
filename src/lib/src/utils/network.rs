use cardano_multiplatform_lib::NetworkIdKind;

pub fn match_network(network: &str) -> NetworkIdKind {
    match network {
        "mainnet" => NetworkIdKind::Mainnet,
        "testnet" => NetworkIdKind::Testnet,
        _ => NetworkIdKind::Testnet,
    }
}

pub fn network_to_string(network: NetworkIdKind) -> String {
    match network {
        NetworkIdKind::Mainnet => "mainnet".to_string(),
        NetworkIdKind::Testnet => "testnet".to_string(),
    }
}
