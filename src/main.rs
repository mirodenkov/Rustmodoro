mod timer;
mod control_panel;

use timer::Timer;

fn main() {
    let mut timer = Timer::new(1); // 1 minute for testing
    control_panel::run_timer(&mut timer);
}
