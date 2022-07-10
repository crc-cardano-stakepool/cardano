#[cfg(test)]
#[ctor::ctor]
fn test_setup() {
    use crate::{setup_env, setup_logger, setup_work_dir, Settings};
    let _ = setup_logger(log::LevelFilter::Trace, false, "../../output.log");
    log::debug!("Setting up tests");
    setup_work_dir().unwrap();
    setup_env().unwrap();
    Settings::show_settings();
    log::info!("Setting up tests succeeded")
}

#[cfg(test)]
#[ctor::dtor]
fn test_teardown() {
    log::debug!("Tearing down tests");
}
