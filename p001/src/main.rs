use lib::runner;

fn main() {
    runner(p001(1000));
}

fn p001(limit: u32) -> u32 {
    (0..limit).collect::<Vec<u32>>().into_iter().filter(|x| (x % 3) == 0 || (x % 5) == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_p001() {
        assert_eq!(p001(10), 23);
    }
}
