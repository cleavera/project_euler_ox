use std::fmt::Display;

pub mod big_int;
pub mod factorial;
pub mod timer;
pub mod fibonacci;
pub mod primes;
pub mod number;
pub mod triangle_numbers;

pub fn runner<F, B>(ans: F)
    where F: FnOnce() -> B, B: Display {
    let timer = timer::Timer::new();
    
    println!("Answer: {}", ans());
    
    timer.print_elapsed();
}
