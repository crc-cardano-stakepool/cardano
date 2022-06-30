#[cfg(test)]
#[ctor::ctor]
fn test_setup() {
    use crate::setup_logger;
    use log::LevelFilter;
    let _ = setup_logger(LevelFilter::Debug, false, "../../output.log");
    log::debug!("Setting up tests");
}

#[cfg(test)]
#[ctor::dtor]
fn test_teardown() {
    log::debug!("Tearing down tests");
}
