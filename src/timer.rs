/*Enum with timer states */
#[derive(Debug, Clone, PartialEq)]
pub enum TimerState {
    Running,
    Paused,
    Reset,
    Finished,
}

pub struct Timer {
    remaining_secs: u64,
    starting_secs: u64,
    state: TimerState,
}

pub trait TimerControl {
    fn tick(&mut self); //&mut self means that it reads and writes the struct
    fn get_remaining(&self) -> u64; // returns how many seconds are remaining
    fn reset(&mut self);
    fn pause(&mut self) -> Result<(), String>;
    fn start(&mut self) -> Result<(), String>;
}

//Constructor
impl Timer {
    pub fn new(minutes: u64) -> Self {
        let secs = minutes * 60;
        Self {
            remaining_secs: secs,
            starting_secs: secs,
            state: TimerState::Reset,
        }
    }

    pub fn state(&self) -> &TimerState {
        &self.state
    }
    pub fn remaining(&self) -> u64 {
        self.remaining_secs
    }
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
    }
    fn reset(&mut self) {
        self.remaining_secs = self.starting_secs; //reseting to starting_secs
        self.state = TimerState::Reset;
    }
    fn pause(&mut self) -> Result<(), String> {
        if self.state != TimerState::Running {
            return Err("Can't pause a timer that isn't running".to_string());
        }
        self.state = TimerState::Paused;
        Ok(())
    }
    fn start(&mut self) -> Result<(), String> {
        if self.state == TimerState::Finished && self.remaining_secs == 0 {
            return Err("Can't start a finished timer".to_string());
        } else if self.state == TimerState::Running {
            return Err("Timer is already running".to_string());
        }
        self.state = TimerState::Running;
        Ok(())
    }
}
