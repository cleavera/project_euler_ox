use lib::runner;

pub struct CollatzSequence {
    n: u64,
}

impl CollatzSequence {
    pub fn new(start: u64) -> CollatzSequence {
        return CollatzSequence { n: start };
    }
}

impl Iterator for CollatzSequence {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.n == 1 {
            return None;
        }

        if (self.n % 2) == 0 {
            self.n = self.n / 2;

            return Some(self.n);
        }

        self.n = (3 * self.n) + 1;

        return Some(self.n);
    }
}

fn main() {
    runner(|| p013(1000000));
}

fn p013(n: u64) -> u64 {
    (2..n)
        .map(|i| (i, CollatzSequence::new(i).count()))
        .max_by(|&(_, s1), &(_, s2)| s1.cmp(&s2))
        .map(|(n, _): (u64, usize)| n)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p013() {
        assert_eq!(p013(10), 9);
    }
}
