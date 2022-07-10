#[cfg(test)]
#[ctor::ctor]
fn test_setup() {
    use crate::{setup_logger, Environment, FileSystem, Settings};
    let _ = setup_logger(log::LevelFilter::Trace, false, "../../output.log");
    log::debug!("Setting up tests");
    FileSystem::setup_work_dir().unwrap();
    Environment::setup_env().unwrap();
    Settings::show_settings();
    log::info!("Setting up tests succeeded")
}

#[cfg(test)]
#[ctor::dtor]
fn test_teardown() {
    log::debug!("Tearing down tests");
}
