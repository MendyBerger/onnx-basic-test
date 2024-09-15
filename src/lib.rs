mod simple_graph;
mod squeeze;

struct MyCliRunner;

impl wasi::exports::cli::run::Guest for MyCliRunner {
    fn run() -> Result<(), ()> {
        WasiLogger::init();

        pollster::block_on(squeeze::run());

        Ok(())
    }
}

wasi::cli::command::export!(MyCliRunner);

use log::LevelFilter;
use log::{Metadata, Record};

pub(crate) struct WasiLogger;

impl WasiLogger {
    pub fn init() {
        static LOGGER: WasiLogger = WasiLogger;
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(LevelFilter::Info);
    }
}

impl log::Log for WasiLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let s = format!("wasi-logger: {} - {}\n", record.level(), record.args());
        let stdout = wasi::cli::stdout::get_stdout();
        stdout.blocking_write_and_flush(s.as_bytes()).unwrap();
    }

    fn flush(&self) {}
}
