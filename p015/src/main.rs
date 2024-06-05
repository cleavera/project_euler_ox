use lib::{big_int::BigInt, factorial::factorial, runner};

fn main() {
    runner(|| p015(20));
}

fn p015(s: u64) -> BigInt {
    BigInt::range((s + 1).into(), (2 * s).into()).product::<BigInt>() / factorial(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p015() {
        assert_eq!(p015(2), 6.into());
        assert_eq!(p015(4), 70.into());
    }
}
