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

//traits are like contracts, they announce what the timer will do and then we need to implement it
trait Countdown {
    fn tick(&mut self); //&mut self means that it reads and writes the struct
    fn get_remaining(&self) -> u64; // returns how many seconds are remaining
}

trait Reset {
    fn reset(&mut self) -> bool;
}

trait Pause {
    fn pause(&mut self) -> bool;
}

trait Start {
    fn start(&mut self) -> bool;
}

///////////////////////////////////////////////////////////////////////////////////////////
/// Implementations
///
///
///
///
///

//pub fn get_state(&self) -> &TimerState {
  //  &self.state
//}

impl Countdown for Timer {
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
}

impl Reset for Timer {
    fn reset(&mut self) -> bool {
        self.remaining_secs = 0; //reseting to zero
        self.state = TimerState::Reset;
        true
    }
}

impl Pause for Timer {
    fn pause(&mut self) -> bool {
        self.state = TimerState::Paused;
        true
    }
}

impl Start for Timer {
    fn start(&mut self) -> bool {
        self.state = TimerState::Running;
        true
    }
}
