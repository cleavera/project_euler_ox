use lib::{runner, primes::Primes};

fn main() {
    runner(|| p007(10001));
}

fn p007(i: u64) -> u64 {
    let mut n = 1;

    for p in Primes::new() {
        if n == i {
            return p;
        }
        
        n = n + 1;
    }
    
    panic!("Gone past an infinite list?");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_p007() {
        assert_eq!(p007(6), 13);
        assert_eq!(p007(7), 17);
        assert_eq!(p007(100), 541);
    }
}
