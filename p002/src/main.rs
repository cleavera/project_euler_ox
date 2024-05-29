use lib::{fibonacci::Fibonacci, runner};


fn main() {
    runner(|| p002(4000000));
}

fn p002(limit: u32) -> u32 {
    Fibonacci::new().take_while(|a| a < &limit).filter(|x| (x % 2) == 0).sum()
}
