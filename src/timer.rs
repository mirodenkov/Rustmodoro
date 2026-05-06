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
    fn is_finished(&mut self) -> bool; // &self means that it only reads
    fn get_remaining(&self) -> u64; // returns how many seconds are remaining
}

trait Reset {
    fn reset(&mut self);
    fn change_reset(&mut self) -> bool;
}

trait Pause {
    fn pause(&self) -> u64;
    fn change_pause(&mut self) -> bool;
}

trait Start{
 
}

///////////////////////////////////////////////////////////////////////////////////////////
/// Implementations
impl Countdown for Timer {
    fn tick(&mut self) {
        if let TimerState::Running = self.state {
            self.remaining_secs -= 1;
        }
    }

    fn is_finished(&mut self) -> bool {
        if self.remaining_secs == 0{
            self.state = TimerState::Finished;
            return true;
        }
        else{
            false
        }
    }
    fn get_remaining(&self) -> u64 {
        self.remaining_secs
        //note to self: tis the same as return self.remaining_secs;
    }
}


