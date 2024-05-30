use std::u64;

pub struct Primes {
    i: u64,
    reverse: bool,
}

impl Primes {
    pub fn new() -> Primes {
        return Primes {
            i: 2,
            reverse: false,
        };
    }

    pub fn under(i: u64) -> Primes {
        return Primes {
            i,
            reverse: true,
        };
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.i <= 1 && self.reverse {
            return None;
        }

        if self.i == 2 {
            self.i = self.i - 1;

            return Some(2);
        }
        

        if self.reverse {
            self.i = self.i - 2;
        } else {
            self.i = self.i + 2;
        }

        while !check_prime(self.i) {
            if self.reverse {
                self.i = self.i - 2;
            } else {
                self.i = self.i + 2;
            }
        }

        Some(self.i)
    }
}

pub fn check_prime(i: u64) -> bool {
    if (i == 2) || (i == 3) {
        return true;
    }

    if i == 1 || i == 4 {
        return false;
    }

    if i % 2 == 0 {
        return false;
    }

    let mut x = 3;

    while x <= (i / x) {
        if (i % x) == 0 {
            return false;
        }

        x = x + 2;
    }

    return true;
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_check_prime() {
        assert_eq!(check_prime(2), true);
        assert_eq!(check_prime(3), true);
        assert_eq!(check_prime(4), false);
        assert_eq!(check_prime(6), false);
        assert_eq!(check_prime(7), true);
        assert_eq!(check_prime(9), false);
        assert_eq!(check_prime(13), true);
        assert_eq!(check_prime(52711), true);
        assert_eq!(check_prime(648391), true);
        assert_eq!(check_prime(174440041), true);
        assert_eq!(check_prime(53982894593057), true);
        assert_eq!(check_prime(222334565193649), true);
        assert_eq!(check_prime(222334565193651), false);
    }

    #[test]
    fn list_primes() {
        let mut primes = Primes::new();

        for i in (1..1000000).filter(|x| check_prime(*x)) {
            assert_eq!(i, primes.next().expect("There is always more primes"));
        }
    }

    #[test]
    fn primes_under() {
        let i = 1000000;
        let mut primes = Primes::under(1000000);

        for i in (i..2).filter(|x| check_prime(*x)) {
            assert_eq!(i, primes.next().expect("There is always more primes"));
        }

        // assert_eq!(primes.next(), None);
    }
}
