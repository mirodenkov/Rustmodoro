use std::thread;
use std::time::Duration;

use crate::timer::{Timer, TimerControl, TimerState};

pub fn run_timer(timer: &mut Timer) {
    if let Err(e) = timer.start() {
        println!("{}", e);
        return;
    }

    loop {
        timer.tick();
        let remaining = timer.get_remaining();
        let minutes = remaining / 60;
        let seconds = remaining % 60;
        println!("{:02}:{:02}", minutes, seconds);

        if *timer.state() == TimerState::Finished {
            println!("Timer finished!");
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
