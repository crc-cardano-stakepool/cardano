use indicatif::{ProgressBar, ProgressStyle};

pub fn spinner(exec_msg: &'static str, tick_strings: &[&str]) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    let spinner_style = ProgressStyle::default_spinner()
        .template("{msg:.green} {spinner:.green}")
        .tick_strings(tick_strings);
    spinner.enable_steady_tick(80);
    spinner.set_style(spinner_style);
    spinner.set_message(exec_msg);
    spinner
}
