use chrono::Local;
use colored::*;
use log::{Level, LevelFilter};

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{time} {target} {arrow} {level} {message}",
                time = Local::now()
                    .format("[%Y-%m-%d] [%H:%M:%S]")
                    .to_string()
                    .bright_black(),
                target = format!("[{}]", record.target().bright_black()),
                arrow = "›".bright_black(),
                level = {
                    let (color, icon, label) = match record.level() {
                        Level::Info => (Color::Blue, "i", "info"),
                        Level::Warn => (Color::Yellow, "⚠", "warning"),
                        Level::Error => (Color::Red, "✖", "error"),
                        Level::Debug => (Color::Blue, "●", "debug"),
                        Level::Trace => (Color::Magenta, "…", "trace"),
                    };
                    format!(
                        "{icon} {label}{margin} ",
                        icon = icon.color(color),
                        label = label.underline().color(color),
                        // max length of label is 7 (warning)
                        margin = " ".repeat(7 - label.len())
                    )
                },
                message = message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
