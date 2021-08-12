use std::process::Command;
use std::process::Stdio;

fn command(command: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed printing to terminal");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
}

fn main() {
    // Build and link IOHK libsodium
    command("git submodule update --init --recursive --force");

    // Build libsodium automatically (as part of rust build)
    #[cfg(not(feature = "libsodium-sys"))]
    {
        let libsodium = autotools::Config::new("contrib/libsodium/").reconf("-vfi").build();
        println!("cargo:rustc-link-search=native={}", libsodium.join("lib").display());
        println!("cargo:rustc-link-lib=static=sodium");
    }

    // Link with libsodium system library
    #[cfg(feature = "libsodium-sys")]
    {
        pkg_config::Config::new().probe("libsodium").unwrap();
    }
}
