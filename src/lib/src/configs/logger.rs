use fern::{
    colors::{Color, ColoredLevelConfig},
    FormatCallback,
};
use log::{LevelFilter, Record};
use std::{fmt::Arguments, path::Path};

pub fn setup_logger<P: AsRef<Path>>(log_level: LevelFilter, is_cli: bool, log_file: P) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Magenta)
        .trace(Color::BrightCyan);

    let make_formatter = |use_colors: bool| {
        move |out: FormatCallback, message: &Arguments, record: &Record| {
            if !is_cli {
                out.finish(format_args!(
                    "[{}] {} [{}:{}] -> {}",
                    if use_colors {
                        colors.color(record.level()).to_string()
                    } else {
                        record.level().to_string()
                    },
                    chrono::Local::now().format("[%d.%m.%Y %H:%M:%S]"),
                    record.file().unwrap_or("?"),
                    record.line().unwrap_or_default(),
                    message
                ))
            } else {
                out.finish(format_args!(
                    "[{}] -> {}",
                    if use_colors {
                        colors.color(record.level()).to_string()
                    } else {
                        record.level().to_string()
                    },
                    message
                ))
            }
        }
    };

    let file_dispatcher = fern::Dispatch::new()
        .format(make_formatter(false))
        .level(log_level)
        .chain(fern::log_file(log_file)?);

    let stdout_dispatcher = fern::Dispatch::new()
        .format(make_formatter(true))
        .level(log_level)
        .chain(fern::Output::call(|record| println!("{}", record.args())));

    fern::Dispatch::new().chain(stdout_dispatcher).chain(file_dispatcher).apply()?;

    Ok(())
}
