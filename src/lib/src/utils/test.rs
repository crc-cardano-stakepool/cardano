#[cfg(test)]
#[ctor::ctor]
fn test_setup() {
    use crate::{setup_env, setup_logger};
    use log::LevelFilter;
    let _ = setup_logger(LevelFilter::Debug, false, "../../output.log");
    log::debug!("Setting up tests");
    setup_env().unwrap();
}

#[cfg(test)]
#[ctor::dtor]
fn test_teardown() {
    log::debug!("Tearing down tests");
}
