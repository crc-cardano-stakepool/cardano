use log::info;

pub fn setup() {
    std::env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
}

pub fn teardown() {
    info!("Cleaning up")
}
