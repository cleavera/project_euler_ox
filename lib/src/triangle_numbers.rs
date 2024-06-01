pub struct TriangleNumbers {
    i: u64,
    n: u64
}

impl TriangleNumbers {
    pub fn new() -> TriangleNumbers {
        return TriangleNumbers {
            i: 1,
            n: 1
        };
    }
}

impl Iterator for TriangleNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.i = self.i + 1;
        self.n = self.n + self.i;

        return Some(self.n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_triangle_numbers() {
        let mut triangle_numbers = TriangleNumbers::new();
        let mut n = 1;

        for i in 2..1000 {
            n = n + i;
            assert_eq!(n, triangle_numbers.next().expect("There is always more triangle numbers"));
        }
    }
}
