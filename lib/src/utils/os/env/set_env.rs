use std::env::set_var;
pub fn set_env(key: &str, value: &str) {
    set_var(key, value);
}

#[cfg(test)]
mod test {
    // use crate::set_env;
    #[test]
    #[ignore]
    fn test_set_env() {
        unimplemented!();
    }
}
