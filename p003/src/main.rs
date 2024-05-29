use lib::{primes::{check_prime, Primes}, runner};

fn main() {
    runner(p003(600851475143));
}

fn p003(i: u64) -> u64 {
    if check_prime(i) {
        return i;
    }

    let mut n = i;
    let mut primes = Primes::new();
    let mut prime = primes.next().unwrap();

    while prime < i {
        if (n % prime) == 0 {
            n = n / prime;

            if check_prime(n) {
                return n;
            }
        }

        prime = primes.next().unwrap();
    }

    panic!("Numbers must always have prime factors");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_p003() {
        assert_eq!(p003(13195), 29);
    }
}
