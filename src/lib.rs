mod simple_graph;
mod squeeze;
mod mnist;

struct MyCliRunner;

impl wasi::exports::cli::run::Guest for MyCliRunner {
    fn run() -> Result<(), ()> {
        std::panic::set_hook(Box::new(|info: _| {
            let s = format!("panic message: {info}");
            let stdout = wasi::cli::stdout::get_stdout();
            stdout.blocking_write_and_flush(s.as_bytes()).unwrap();
        }));

        WasiLogger::init();

        log::info!("\nchoose example\n1: simple graph\n2: squeeze\n3: mnist\n");
        let stdin = wasi::cli::stdin::get_stdin();
        let input = stdin.blocking_read(1).unwrap();
        let input = String::from_utf8(input).unwrap();
        log::info!("{input}");
        match input.as_str() {
            "1" => {
                pollster::block_on(simple_graph::run()).unwrap();
            },
            "2" => {
                pollster::block_on(squeeze::run());
            },
            "3" => {
                pollster::block_on(mnist::run());
            }
            s => {
                todo!("{s} not found")
            }
        };

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
