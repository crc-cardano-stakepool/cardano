use std::env::set_var;
pub fn set_env(key: &str, value: &str) {
    set_var(key, value);
}
