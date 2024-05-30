use lib::{primes::Primes, runner};

fn main() {
    runner(|| p010(2000000));
}

fn p010(n: u64) -> u64 {
    Primes::under(n).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p010() {
        assert_eq!(p010(10), 17);
    }
}
