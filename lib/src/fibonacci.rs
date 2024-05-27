pub struct Fibonacci {
    current: u32,
    next_value: u32,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { current: 0, next_value: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let current = self.current;

        self.current = self.next_value;
        self.next_value = self.current + current;

        Some(self.next_value)
    }
}
