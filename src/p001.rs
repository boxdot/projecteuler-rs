#[allow(dead_code)]
fn sum_of_mult_3_and_5_naive(n: u64) -> u64 {
    (1..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

fn sum_div_by(n: u64, p: u64) -> u64 {
    let num_div_by = (n - 1) / p;
    p * num_div_by * (num_div_by + 1) / 2
}

fn sum_of_mult_3_and_5(n: u64) -> u64 {
    sum_div_by(n, 3) + sum_div_by(n, 5) - sum_div_by(n, 15)
}

pub fn solve() -> u64 {
    sum_of_mult_3_and_5(1000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_mult_3_and_5() {
        assert!(23 == sum_of_mult_3_and_5_naive(10));
        assert!(23 == sum_of_mult_3_and_5(10));
    }
}
