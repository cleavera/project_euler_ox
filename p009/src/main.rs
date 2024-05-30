use std::u64;

use lib::runner;

fn main() {
    runner(|| p009(1000));
}

fn p009(n: u64) -> u64 {
    for i in 1..n {
        for j in i..(n - i) {
            let k2 = (i * i) + (j * j);
            let k = f64::sqrt(k2 as f64);

            if k == k.floor() {
                if (i + j + (k as u64)) == n {
                    return i * j * (k as u64);
                }
            }
        }
    }

    panic!("They lied");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p009() {
        assert_eq!(p009(12), 60);
    }
}
