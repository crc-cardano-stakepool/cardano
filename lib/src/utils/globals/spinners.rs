use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref SPINNERS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("arrows", vec!["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"]);
        map
    };
}