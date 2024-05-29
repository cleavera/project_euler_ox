use lib::runner;

fn main() {
    runner(|| p005(20));
}

fn p005(i: u64) -> u64 {
    let mut inc = 1;
    let mut n = 1;

    for j in 2..=i {
       while (n % j) != 0 {
           n = n + inc;
       }

       inc = n;
    }

    return n;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_p005() {
        assert_eq!(p005(10), 2520);
    }
}
