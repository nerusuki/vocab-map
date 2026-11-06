use iced::Theme;
use wasm_bindgen::prelude::*;

mod counter;
mod word_map;

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
    iced::application(Counter::default, Counter::update, Counter::view)
        .theme(|state: &Counter| Theme::Dracula)
        .run()
}
