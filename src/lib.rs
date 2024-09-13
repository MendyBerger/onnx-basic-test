wit_bindgen::generate!({
    path: "wit",
    world: "example:example/example",
});

mod simple_graph;
mod squeeze;

// // #[wasm_bindgen_test]
// #[no_mangle]
// pub extern fn start(example: i32, b: i32) {
//     println!("Hello, world! {example}");
//     black_box(example);
//     black_box(b);
//     // #[cfg(not(target_arch = "wasm32"))]
//     // {
//     //     env_logger::init();
//         pollster::block_on(run()).unwrap();
//     // }
//     // #[cfg(target_arch = "wasm32")]
//     // {
//     //     std::panic::set_hook(Box::new(console_error_panic_hook::hook));
//     //     console_log::init().expect("could not initialize logger");
//     //     wasm_bindgen_futures::spawn_local(run());
//     // }
// }


struct Example;

impl Guest for Example {
    fn start(example: String) {
        WasiLogger::init();

        std::panic::set_hook(Box::new(|info: _| {
            print(&format!("panic message: {info}"));
        }));

        print(&example);

        // pollster::block_on(simple_graph::run()).unwrap();
        pollster::block_on(squeeze::run());
    }
}

export!(Example);



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
        print(&format!(
            "wasi-logger: {} - {}",
            record.level(),
            record.args()
        ));
    }

    fn flush(&self) {}
}
