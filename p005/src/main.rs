use lib::runner;

fn main() {
    runner(|| p005(20));
}

fn p005(i: u64) -> u64 {
    let mut n = 6;

    loop {
        for j in 2..i + 1 {
            if (n % j) != 0 {
                break;
            }

            if i == j {
                return n;
            }
        }

        n = n + 6;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_p005() {
        assert_eq!(p005(10), 2520);
    }
}
