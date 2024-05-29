pub fn is_palindrome(i: u64) -> bool {
    i == reverse(i) 
}

pub fn reverse(mut i: u64) -> u64 {
    let mut r = 0;

    while i > 0 {
        r = (r * 10) + (i % 10);
        i = i / 10;
    }

    return r;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn reverse_test() {
        assert_eq!(reverse(13195), 59131);
        assert_eq!(reverse(133769420), 24967331);
        assert_eq!(reverse(8008), 8008);
        assert_eq!(reverse(195121591), 195121591);
        assert_eq!(reverse(12345), 54321);
        assert_eq!(reverse(81912), 21918);
    }
    
    #[test]
    fn palindrome_test() {
        assert_eq!(is_palindrome(13195), false);
        assert_eq!(is_palindrome(133769420), false);
        assert_eq!(is_palindrome(8008), true);
        assert_eq!(is_palindrome(195121591), true);
        assert_eq!(is_palindrome(12345), false);
        assert_eq!(is_palindrome(81912), false);
    }
}
