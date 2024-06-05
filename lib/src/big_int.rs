use std::{
    cmp::Ordering, fmt, iter::Product, ops::{Add, Div, Mul, Sub}
};

pub struct Range {
    start: BigInt,
    end: BigInt,
    current: Option<BigInt>,
}

impl Iterator for Range {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            None => {
                self.current = Some(self.start.clone());
            }
            Some(cur) if cur == &self.end => {
                return None;
            }
            Some(cur) => {
                self.current = Some(cur.clone() + BigInt::from_str("1"));
            }
        }

        self.current.clone()
    }
}

#[derive(Debug)]
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

    pub fn subtract(a: BigInt, b: BigInt) -> BigInt {
        let mut result = Vec::new();
        let mut borrow = 0;

        for i in 0..a.digits.len() {
            let digit_a = a.digits[i];
            let digit_b = b.digits.get(i).cloned().unwrap_or(0);
            let mut diff = digit_a as i32 - digit_b as i32 - borrow;

            if diff < 0 {
                diff += 10;
                borrow = 1;
            } else {
                borrow = 0;
            }

            result.push(diff as u32);
        }

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
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

    pub fn divide(a: BigInt, b: BigInt) -> BigInt {
        let mut quotient = BigInt::from(0);
        let mut remainder = BigInt::from(0);

        for &digit in a.digits.iter().rev() {
            remainder = (remainder * 10.into()) + digit.into();
            quotient = quotient * 10.into();

            let mut q = 0;

            while remainder >= b {
                remainder = BigInt::subtract(remainder.clone(), b.clone());
                q += 1;
            }

            quotient = quotient + q.into();
        }

        quotient
    }

    pub fn range(start: BigInt, end: BigInt) -> Range {
        Range {
            start,
            end,
            current: None,
        }
    }

    pub fn range_exclusive(start: BigInt, end: BigInt) -> Range {
        Range {
            start,
            end: end - BigInt::from_str("1"),
            current: None,
        }
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

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        BigInt::add(self, rhs)
    }
}

impl Sub for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        BigInt::subtract(self, rhs)
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        BigInt::multiply(self, rhs)
    }
}

impl Div for BigInt {
    type Output = BigInt;

    fn div(self, rhs: Self) -> Self::Output {
        BigInt::divide(self, rhs)
    }
}

impl Product for BigInt {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(1.into(), |t, n| t * n)
    }
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        BigInt::from_str(&self.to_str())
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut other_digits = other.digits.iter().rev();

        if self.eq(other) {
            return Ordering::Equal;
        }

        if self.digits.len() != other_digits.len() {
            return self.digits.len().cmp(&other_digits.len());
        }

        for d in self.digits.iter().rev() {
            let d2 = other_digits.next().unwrap_or(&0);

            if d != d2 {
                return d.cmp(&d2);
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.to_str() == other.to_str()
    }
}

impl Eq for BigInt {}

impl From<u64> for BigInt {
    fn from(num: u64) -> BigInt {
        let digits = num
            .to_string()
            .chars()
            .rev()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        BigInt { digits }
    }
}

impl From<u32> for BigInt {
    fn from(num: u32) -> BigInt {
        let digits = num
            .to_string()
            .chars()
            .rev()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        BigInt { digits }
    }
}

impl From<i32> for BigInt {
    fn from(num: i32) -> BigInt {
        let digits = num
            .to_string()
            .chars()
            .rev()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        BigInt { digits }
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
        assert_eq!("20", BigInt::from(20).to_str());
        assert_eq!("20123", BigInt::from(20123).to_str());
    }

    #[test]
    fn test_big_int_multiply() {
        assert_eq!(
            "60",
            (BigInt::from_str("5") * BigInt::from_str("12")).to_str()
        );
        assert_eq!(
            "1524157875019052100",
            (BigInt::from_str("1234567890") * BigInt::from_str("1234567890")).to_str()
        );
    }

    #[test]
    fn test_big_int_add() {
        assert_eq!(
            "17",
            (BigInt::from_str("5") + BigInt::from_str("12")).to_str()
        );
        assert_eq!(
            "2469135780",
            (BigInt::from_str("1234567890") + BigInt::from_str("1234567890")).to_str()
        );
    }

    #[test]
    fn test_subtract() {
        assert_eq!(
            (BigInt::from_str("10") - BigInt::from_str("3")).to_str(),
            "7"
        );
        assert_eq!(
            (BigInt::from_str("100") - BigInt::from_str("1")).to_str(),
            "99"
        );
        assert_eq!(
            (BigInt::from_str("12345") - BigInt::from_str("12345")).to_str(),
            "0"
        );
        assert_eq!(
            (BigInt::from_str("100000000000000000000") - BigInt::from_str("99999999999999999999"))
                .to_str(),
            "1"
        );
        assert_eq!(
            (BigInt::from_str("100020003000") - BigInt::from_str("20003000")).to_str(),
            "100000000000"
        );
    }

    #[test]
    fn test_divide() {
        assert_eq!(
            (BigInt::from_str("10") / BigInt::from_str("2")).to_str(),
            "5"
        );
        assert_eq!(
            (BigInt::from_str("25") / BigInt::from_str("4")).to_str(),
            "6"
        );
        assert_eq!(
            (BigInt::from_str("100") / BigInt::from_str("4")).to_str(),
            "25"
         );
        assert_eq!(
            (BigInt::from_str("12345") / BigInt::from_str("12345")).to_str(),
            "1"
        );
        assert_eq!(
            (BigInt::from_str("10") / BigInt::from_str("3")).to_str(),
            "3"
        );
        assert_eq!(
            ((BigInt::from(8) * BigInt::from(7) * BigInt::from(6) * BigInt::from(5))
                / (BigInt::from(4) * BigInt::from(3) * BigInt::from(2) * BigInt::from(1)))
            .to_str(),
            "70"
        );
    }

    #[test]
    fn test_big_int_range_exclusive() {
        let mut int_range = 4..123;
        let big_int_range = BigInt::range_exclusive(BigInt::from_str("4"), BigInt::from_str("123"));

        for i in big_int_range {
            let j = int_range.next().unwrap();

            println!("{}, {}", i.to_str(), j);
            assert_eq!(j.to_string(), i.to_str());
        }
    }

    #[test]
    fn test_big_int_range_inclusive() {
        let mut int_range = 4..=123;
        let big_int_range = BigInt::range(BigInt::from_str("4"), BigInt::from_str("123"));

        for i in big_int_range {
            let j = int_range.next().unwrap();

            println!("{}, {}", i.to_str(), j);
            assert_eq!(j.to_string(), i.to_str());
        }
    }

    #[test]
    fn test_big_int_product() {
        assert_eq!(BigInt::from(6), BigInt::range(1.into(), 3.into()).product());
    }

    #[test]
    fn test_compare() {
        assert_eq!(BigInt::from(6) > BigInt::from(4), true);
        assert_eq!(BigInt::from(6) >= BigInt::from(4), true);
        assert_eq!(BigInt::from(6) == BigInt::from(4), false);
        assert_eq!(BigInt::from(6) <= BigInt::from(4), false);
        assert_eq!(BigInt::from(6) < BigInt::from(4), false);

        assert_eq!(BigInt::from(6) > BigInt::from(6), false);
        assert_eq!(BigInt::from(6) >= BigInt::from(6), true);
        assert_eq!(BigInt::from(6) == BigInt::from(6), true);
        assert_eq!(BigInt::from(6) <= BigInt::from(6), true);
        assert_eq!(BigInt::from(6) < BigInt::from(6), false);

        assert_eq!(BigInt::from(4) > BigInt::from(6), false);
        assert_eq!(BigInt::from(4) >= BigInt::from(6), false);
        assert_eq!(BigInt::from(4) == BigInt::from(6), false);
        assert_eq!(BigInt::from(4) <= BigInt::from(6), true);
        assert_eq!(BigInt::from(4) < BigInt::from(6), true);

        assert_eq!(BigInt::from_str("1908983572339388141") > BigInt::from_str("243290200817664000"), true);
        assert_eq!(BigInt::from_str("1908983572339388141") >= BigInt::from_str("243290200817664000"), true);
        assert_eq!(BigInt::from_str("1908983572339388141") == BigInt::from_str("243290200817664000"), false);
        assert_eq!(BigInt::from_str("1908983572339388141") <= BigInt::from_str("243290200817664000"), false);
        assert_eq!(BigInt::from_str("1908983572339388141") < BigInt::from_str("243290200817664000"), false);

        assert_eq!(BigInt::from_str("1908983572339388141") > BigInt::from_str("2432902008176640000"), false);
        assert_eq!(BigInt::from_str("1908983572339388141") >= BigInt::from_str("2432902008176640000"), false);
        assert_eq!(BigInt::from_str("1908983572339388141") == BigInt::from_str("2432902008176640000"), false);
        assert_eq!(BigInt::from_str("1908983572339388141") <= BigInt::from_str("2432902008176640000"), true);
        assert_eq!(BigInt::from_str("1908983572339388141") < BigInt::from_str("2432902008176640000"), true);

        assert_eq!(BigInt::from_str("243290200817664000") > BigInt::from_str("1908983572339388141"), false);
        assert_eq!(BigInt::from_str("243290200817664000") >= BigInt::from_str("1908983572339388141"), false);
        assert_eq!(BigInt::from_str("243290200817664000") == BigInt::from_str("1908983572339388141"), false);
        assert_eq!(BigInt::from_str("243290200817664000") <= BigInt::from_str("1908983572339388141"), true);
        assert_eq!(BigInt::from_str("243290200817664000") < BigInt::from_str("1908983572339388141"), true);
    }
}
