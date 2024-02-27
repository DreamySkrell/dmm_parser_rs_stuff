mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn set_up_logging() {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                chrono::Local::now().format("%H:%M:%S.%3f"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::Output::call(|record| {
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("logs")
                .unwrap()
                .append_with_str_1(&format!("{}\n", record.args()))
                .unwrap();
        }))
        .apply()
        .unwrap();
    log::info!("logging initialized");
}

#[wasm_bindgen]
pub fn autopipe(origin_str: String) -> String {
    let parsed = dmm_parser_rs::dmmr::parse(&origin_str);
    let umm = dmm_parser_rs::dmmr::unpack(&parsed);
    let umm = dmm_parser_rs::autopipe::autopipe(umm);
    let repacked = dmm_parser_rs::dmmr::pack(&umm);
    let result_str = dmm_parser_rs::dmmr::print(&repacked);
    result_str
}
