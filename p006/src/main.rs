use lib::runner;

fn main() {
    runner(|| p006(100));
}

fn p006(i: u64) -> u64 {
    let sum_of_squares: u64 = (1..=i).map(|x| x * x).sum();
    let sum: u64 = (1..=i).sum();
    let square_of_sums: u64 = sum * sum;

    if sum_of_squares > square_of_sums {
        return sum_of_squares - square_of_sums;
    }

    return square_of_sums - sum_of_squares;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_p006() {
        assert_eq!(p006(10), 2640);
    }
}
