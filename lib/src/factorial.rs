use crate::big_int::BigInt;

pub fn factorial(n: u64) -> BigInt {
    BigInt::range(1.into(), n.into()).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_factorial() {
        assert_eq!(BigInt::from(6), factorial(3));
        assert_eq!(BigInt::from(24), factorial(4));
        assert_eq!(BigInt::from(120), factorial(5));
    }
}
