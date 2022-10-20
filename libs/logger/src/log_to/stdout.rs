use std::io;

use fern::colors::{Color, ColoredLevelConfig};

pub struct LogToStdout;

impl LogToStdout {
    pub fn init() -> fern::Dispatch {
        let color = ColoredLevelConfig::new()
            .error(Color::BrightRed)
            .info(Color::Green)
            .debug(Color::Yellow)
            .trace(Color::Blue)
            .warn(Color::BrightMagenta);

        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{time} | {level:<5} - [{local}] => {message}",
                    time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    // level
                    level = color.color(record.level()),
                    // local
                    local = record.target(),
                ))
            })
            .chain(io::stdout())
    }
}
