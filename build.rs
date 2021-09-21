// Source: https://github.com/AndrewWestberg/cncli/blob/develop/build.rs
use anyhow::{anyhow, Result};
use console::{Color, Emoji, Style, Term};
use std::process::{Command as Cmd, Stdio};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    setup_packages().await?;
    // Build and link IOHK libsodium
    async_command("git submodule update --init --recursive").await?;
    // Build libsodium automatically (as part of rust build)
    #[cfg(not(feature = "libsodium-sys"))]
    {
        let libsodium = autotools::Config::new("contrib/libsodium").reconf("-vfi").build();
        println!("cargo:rustc-link-search=native={}", libsodium.join("lib").display());
        println!("cargo:rustc-link-lib=static=sodium");
    }
    println!("cargo:return-if-changed=build.rs");
    Ok(())
}

pub async fn async_command(command: &str) -> Result<String> {
    let child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::inherit())
        .spawn()?
        .wait_with_output()
        .await;
    match child {
        Ok(output) => Ok(String::from(String::from_utf8_lossy(&output.stdout))),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub async fn async_command_pipe(command: &str) -> Result<String> {
    let process = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .output()
        .await;
    match process {
        Ok(output) => Ok(String::from(String::from_utf8_lossy(&output.stdout))),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub fn print(color: &str, output: &str, emoji: Emoji<'_, '_>) -> Result<()> {
    match to_color(color) {
        Color::Cyan => {
            let cyan = format!("{} {}", Style::new().cyan().apply_to(output), emoji);
            Term::stdout().write_line(&cyan)?;
        }
        Color::Blue => {
            let blue = format!("{} {}", Style::new().blue().apply_to(output), emoji);
            Term::stdout().write_line(&blue)?;
        }
        Color::Black => {
            let black = format!("{} {}", Style::new().black().apply_to(output), emoji);
            Term::stdout().write_line(&black)?;
        }
        Color::Red => {
            let red = format!("{} {}", Style::new().red().apply_to(output), emoji);
            Term::stdout().write_line(&red)?;
        }
        Color::Green => {
            let green = format!("{} {}", Style::new().green().apply_to(output), emoji);
            Term::stdout().write_line(&green)?;
        }
        Color::Yellow => {
            let yellow = format!("{} {}", Style::new().yellow().apply_to(output), emoji);
            Term::stdout().write_line(&yellow)?;
        }
        Color::Magenta => {
            let magenta = format!("{} {}", Style::new().magenta().apply_to(output), emoji);
            Term::stdout().write_line(&magenta)?;
        }
        Color::White => {
            let white = format!("{} {}", Style::new().white().apply_to(output), emoji);
            Term::stdout().write_line(&white)?;
        }
        _ => {
            let white = format!("{} {}", Style::new().white().apply_to(output), emoji);
            Term::stdout().write_line(&white)?;
        }
    };
    Ok(())
}

fn to_color(color: &str) -> Color {
    match color {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        _ => Color::White,
    }
}

pub async fn pipe(command: &str, pipe_command: &str) -> Result<String> {
    let mut child = Cmd::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    if let Some(output) = child.stdout.take() {
        let process = Cmd::new("sh")
            .arg("-c")
            .arg(pipe_command)
            .stdin(output)
            .stdout(Stdio::piped())
            .spawn()?;
        let process = process.wait_with_output();
        match process {
            Ok(output) => Ok(String::from(String::from_utf8_lossy(&output.stdout))),
            Err(e) => Err(anyhow!("{}", e)),
        }
    } else {
        Err(anyhow!("Failed executing piped command"))
    }
}

pub async fn check_distro() -> Result<String> {
    println!("Checking distro");
    let helper_string = "'{print $2}'";
    let cmd = format!("cat /etc/*ease | grep DISTRIB_ID | awk -F '=' {}", helper_string);
    let distro = async_command_pipe(&cmd).await;
    match distro {
        Ok(distro) => {
            let msg = format!("Detected {}", distro.trim());
            print("green", &msg, Emoji("", ""))?;
            Ok(distro)
        }
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub async fn check_platform() -> Result<String> {
    println!("Checking platform");
    let platform = async_command_pipe("uname").await;
    match platform {
        Ok(platform) => Ok(platform),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub async fn setup_packages() -> Result<()> {
    let output = check_platform().await?;
    let platform = output.as_str().trim();
    match platform {
        "linux" | "Linux" => {
            print("green", "Detected Linux", Emoji("", ""))?;
            let output = check_distro().await?;
            let distro = output.as_str().trim();
            install_distro_packages(distro).await?;
        }
        "darwin" | "Darwin" => {
            print("green", "Detected macOS", Emoji("", ""))?;
            install_mac_packages().await?
        }
        _ => panic!("Unsupported platform: {}", platform),
    }
    Ok(())
}

pub async fn install_distro_packages(distro: &str) -> Result<()> {
    println!("Installing {} dependencies", distro);
    match distro {
        "Ubuntu" | "Debian" => {
            let package_manager = "apt";
            let packages = vec![
                "curl",
                "automake",
                "build-essential",
                "pkg-config",
                "libffi-dev",
                "libgmp-dev",
                "libssl-dev",
                "libtinfo-dev",
                "libsystemd-dev",
                "zlib1g-dev",
                "make",
                "g++",
                "tmux",
                "git",
                "jq",
                "wget",
                "libncursesw5",
                "libtool",
                "autoconf",
            ];
            install_packages(package_manager, packages).await?;
        }
        "Fedora" | "Hat" | "CentOs" => {
            let package_manager = "yum";
            let packages = vec![
                "curl",
                "git",
                "gcc",
                "gcc-c++",
                "tmux",
                "gmp-devel",
                "make",
                "tar",
                "xz",
                "wget",
                "zlib-devel",
                "libtool",
                "autoconf",
                "systemd-devel",
                "ncurses-devel",
                "ncurses-compat-libs",
            ];
            install_packages(package_manager, packages).await?;
        }
        _ => panic!("Unsupported distro: {}", distro),
    }
    Ok(())
}

pub async fn install_packages(package_manager: &str, packages: Vec<&str>) -> Result<()> {
    println!("Updating");
    let cmd = format!("{} update -y", package_manager);
    async_command_pipe(&cmd).await?;
    print("green", "Finished updating", Emoji("", ""))?;
    for package in packages {
        check_package(package_manager, package).await?;
    }
    print("green", "Successfully installed packages", Emoji("", ""))?;
    Ok(())
}

pub async fn check_package(package_manager: &str, package: &str) -> Result<()> {
    println!("Checking for {}", package);
    match package_manager {
        "apt" => {
            let cmd = format!("dpkg -s {}", package.trim());
            let piped_cmd = "grep installed";
            let output = pipe(&cmd, piped_cmd).await;
            match output {
                Ok(result) => {
                    if result.trim().is_empty() {
                        install_package(package_manager, package).await?;
                    } else {
                        let msg = format!("{} is installed", package);
                        print("green", &msg, Emoji("", ""))?;
                    }
                }
                Err(_) => {
                    let msg = format!("Failed checking {}", package);
                    print("red", &msg, Emoji("", ""))?
                }
            }
        }
        "yum" => {
            let cmd = format!("rpm -q {}", package);
            let output = async_command_pipe(&cmd).await;
            match output {
                Ok(_) => {
                    let msg = format!("{} is installed", package);
                    print("green", &msg, Emoji("", ""))?;
                }
                Err(_) => install_package(package_manager, package).await?,
            }
        }
        _ => {
            let msg = format!("Failed checking {}", package);
            print("red", &msg, Emoji("", ""))?
        }
    };
    Ok(())
}

pub async fn install_package(package_manager: &str, package: &str) -> Result<()> {
    let msg = format!("{} is not installed", package);
    print("red", &msg, Emoji("", ""))?;
    let cmd = format!("{} install {}", package_manager, package);
    let process = async_command_pipe(&cmd).await;
    match process {
        Ok(_) => {
            let msg = format!("Installed {}", package);
            print("green", &msg, Emoji("", ""))?;
            Ok(())
        }
        Err(e) => Err(anyhow!("Failed installing {} with error: {}", package, e)),
    }
}

pub async fn install_mac_packages() -> Result<()> {
    Ok(())
}
