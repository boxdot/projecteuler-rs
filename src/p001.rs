fn sum_of_mult_3_and_5(n: u64) -> u64 {
    (1..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

pub fn solve() -> u64 {
    sum_of_mult_3_and_5(1000)
}

#[cfg(test)]
mod test {
    use super::sum_of_mult_3_and_5;

    #[test]
    fn test_sum_of_mult_3_and_5() {
        assert!(23 == sum_of_mult_3_and_5(10));
    }
}
