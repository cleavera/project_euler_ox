use std::fmt::Display;

pub mod timer;
pub mod fibonacci;

pub fn runner(ans: impl Display) -> () {
    let timer = timer::Timer::new();
    
    println!("Answer: {}", ans);
    
    timer.print_elapsed();
}
