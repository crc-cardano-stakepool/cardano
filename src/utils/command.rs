use std::process::Stdio;
use tokio::process::Command;

pub async fn command(cmd: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .await;

    match output {
        Ok(output) => println!("{}", String::from_utf8_lossy(&output.stdout)),
        Err(err) => panic!("Error: {}", err),
    };
}
