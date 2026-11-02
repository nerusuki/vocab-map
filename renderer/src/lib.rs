use wasm_bindgen::prelude::*;

mod counter;

use counter::Counter;

#[cfg(test)]
fn main() {
    launch_interface().ok();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn launch() {
    alert(&format!("launching interface!"));
    launch_interface().ok();
}

fn launch_interface() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
