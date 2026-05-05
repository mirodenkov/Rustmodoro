
/*Enum with timer states */
enum TimerState{
    Running,
    Paused,
    Stopped,
}

pub struct Timer{
    pub remaining_secs: u64,
    pub state: TimerState,
}

//traits are like contracts, they announce what the timer will do and then we need to implement it
trait Countdown{
    fn tick(&mut self); //&mut self means that it reads and writes the struct
    fn is_finished(&self) -> bool; // &self means that it only reads
    fn remaining(&self) -> u64;
}

impl Countdown for Timer{

    fn tick(&mut self){
        if self.remaining_secs > 0{
            self.remaining_secs -=1;
        }
    }

    fn is_finished(&self){
        if self.remaining_secs == 0{
            self.is_finished = true;
        }
    }
}

