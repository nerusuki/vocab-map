mod counter;

use counter::Counter;

#[cfg(test)]
fn main() {
    launch_interface().ok();
}

fn launch_interface() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
