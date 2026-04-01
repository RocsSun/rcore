use log::{Level, LevelFilter};

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() >= Level::Debug
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Debug => 32,
            Level::Error => 31,
            Level::Info => 34,
            Level::Trace => 90,
            Level::Warn => 93,
        };

        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        )
    }

    fn flush(&self) {}
}

pub fn init() {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("Debug") => LevelFilter::Debug,
        Some("Error") => LevelFilter::Error,
        Some("Info") => LevelFilter::Info,
        Some("Trace") => LevelFilter::Trace,
        Some("Warn") => LevelFilter::Warn,
        _ => LevelFilter::Info,
    });
}
