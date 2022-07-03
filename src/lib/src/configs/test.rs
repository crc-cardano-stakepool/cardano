#[cfg(test)]
#[ctor::ctor]
fn test_setup() {
    use crate::{setup_env, setup_logger, setup_work_dir, show_settings};
    use log::LevelFilter;
    let _ = setup_logger(LevelFilter::Debug, false, "../../output.log");
    log::debug!("Setting up tests");
    setup_work_dir().unwrap();
    setup_env().unwrap();
    show_settings();
    log::info!("Setting up tests succeeded")
}

#[cfg(test)]
#[ctor::dtor]
fn test_teardown() {
    log::debug!("Tearing down tests");
}
