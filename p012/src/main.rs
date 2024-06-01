use lib::{number::divisors, runner, triangle_numbers::TriangleNumbers};

fn main() {
    runner(|| p012(500));
}

fn p012(n: usize) -> u64 {
    for x in TriangleNumbers::new() {
        if divisors(x).len() > n {
            return x;
        }
    }

    panic!("Gotta happen eventually");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p012() {
        assert_eq!(p012(5), 28);
    }
}
