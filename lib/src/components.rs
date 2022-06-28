use crate::{
    CARDANO_CLI, CARDANO_DB, CARDANO_DB_RELEASE_URL, CARDANO_DB_URL, CARDANO_NODE, CARDANO_NODE_RELEASE_URL,
    CARDANO_NODE_URL, CARDANO_WALLET, CARDANO_WALLET_RELEASE_URL, CARDANO_WALLET_URL,
};

#[derive(Debug, Clone, Copy)]
pub enum Component {
    Cli,
    Node,
    Wallet,
    Db,
}

pub fn get_component_name(component: Component) -> &'static str {
    match component {
        Component::Cli => CARDANO_CLI,
        Component::Node => CARDANO_NODE,
        Component::Wallet => CARDANO_WALLET,
        Component::Db => CARDANO_DB,
    }
}

pub fn get_component_repo_url(component: Component) -> &'static str {
    match component {
        Component::Cli => CARDANO_NODE_URL,
        Component::Node => CARDANO_NODE_URL,
        Component::Wallet => CARDANO_WALLET_URL,
        Component::Db => CARDANO_DB_URL,
    }
}

pub fn get_component_release_url(component: Component) -> &'static str {
    match component {
        Component::Cli => CARDANO_NODE_RELEASE_URL,
        Component::Node => CARDANO_NODE_RELEASE_URL,
        Component::Wallet => CARDANO_WALLET_RELEASE_URL,
        Component::Db => CARDANO_DB_RELEASE_URL,
    }
}
