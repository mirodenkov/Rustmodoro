/*Enum with timer states */
pub enum TimerState {
    Running,
    Paused,
    Reset,
    Finished,
}

pub struct Timer {
    pub remaining_secs: u64,
    pub starting_secs: u64,
    pub state: TimerState,
}

trait TimerControl {
    fn tick(&mut self); //&mut self means that it reads and writes the struct
    fn get_remaining(&self) -> u64; // returns how many seconds are remaining
    fn reset(&mut self) -> bool;
    fn pause(&mut self) -> bool;
    fn start(&mut self) -> bool;
}


impl TimerControl for Timer {
    fn tick(&mut self) {
        if matches!(self.state, TimerState::Running) {
            self.remaining_secs = self.remaining_secs.saturating_sub(1);
            if self.remaining_secs == 0 {
                self.state = TimerState::Finished;
            }
        }
    }
    fn get_remaining(&self) -> u64 {
        self.remaining_secs
        //note to self: tis the same as return self.remaining_secs;
    }
    fn reset(&mut self) -> bool {
        self.remaining_secs = 0; //reseting to zero
        self.state = TimerState::Reset;
        true
    }
    fn pause(&mut self) -> bool {
        self.state = TimerState::Paused;
        true
    }
    fn start(&mut self) -> bool {
        self.state = TimerState::Running;
        true
    }
}