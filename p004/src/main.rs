use lib::{number::is_palindrome, runner};

fn main() {
    runner(|| p004(999));
}

fn p004(i: u64) -> u64 {
    let mut x = i * i;

    while x > 0 {
        if is_palindrome(x) {
            let mut j = i;
            let k = x / j;
            
            while j > k {
                if (x % j) == 0 {
                    return x;
                }

                j = j - 1;
            }
        }

        x = x - 1;
    }

    panic!("wtf mate");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_p004() {
        assert_eq!(p004(99), 9009);
    }
}
