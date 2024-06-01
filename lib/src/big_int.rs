pub struct BigInt {
    digits: Vec<u32>,
}

impl BigInt {
    pub fn from_str(s: &str) -> BigInt {
        let digits = s
            .chars()
            .rev()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        return BigInt { digits };
    }

    pub fn add(a: BigInt, b: BigInt) -> BigInt {
        let mut result = Vec::new();
        let mut carry = 0;
        let max_len = a.digits.len().max(b.digits.len());

        for i in 0..max_len {
            let digit_a = a.digits.get(i).cloned().unwrap_or(0);
            let digit_b = b.digits.get(i).cloned().unwrap_or(0);
            let sum = digit_a + digit_b + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }

        if carry > 0 {
            result.push(carry);
        }

        BigInt { digits: result }
    }

    pub fn multiply(a: BigInt, b: BigInt) -> BigInt {
        let mut result = vec![0; a.digits.len() + b.digits.len()];

        for (i, &digit_a) in a.digits.iter().enumerate() {
            for (j, &digit_b) in b.digits.iter().enumerate() {
                result[i + j] += digit_a * digit_b;
                if result[i + j] >= 10 {
                    result[i + j + 1] += result[i + j] / 10;
                    result[i + j] %= 10;
                }
            }
        }

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        BigInt { digits: result }
    }

    pub fn to_str(&self) -> String {
        self.digits
            .iter()
            .rev()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_int() {
        assert_eq!("1234567890", BigInt::from_str("1234567890").to_str());
        assert_eq!(
            "709832741234567890",
            BigInt::from_str("709832741234567890").to_str()
        );
        assert_eq!(
            "12892986124891643297843219846976431978463214785348690143",
            BigInt::from_str("12892986124891643297843219846976431978463214785348690143").to_str()
        );
    }

    #[test]
    fn test_big_int_multiply() {
        assert_eq!("60", BigInt::multiply(BigInt::from_str("5"), BigInt::from_str("12")).to_str());
        assert_eq!("1524157875019052100", BigInt::multiply(BigInt::from_str("1234567890"), BigInt::from_str("1234567890")).to_str());
    }

    #[test]
    fn test_big_int_add() {
        assert_eq!("17", BigInt::add(BigInt::from_str("5"), BigInt::from_str("12")).to_str());
        assert_eq!("2469135780", BigInt::add(BigInt::from_str("1234567890"), BigInt::from_str("1234567890")).to_str());
    }
}
