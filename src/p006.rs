/// Sum square difference
#[allow(dead_code)]
fn sum_of_squares(n: u64) -> u64 {
    (1..n + 1).map(|x| x * x).sum()
}

fn sum_of_squares_const(n: u64) -> u64 {
    n * (2 * n + 1) * (n + 1) / 6
}

fn square_of_sum(n: u64) -> u64 {
    let sum = n * (n + 1) / 2;
    sum * sum
}

pub fn solve() -> u64 {
    square_of_sum(100) - sum_of_squares_const(100)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(10), 385);
        assert_eq!(sum_of_squares_const(10), 385);
    }

    #[test]
    fn test_square_of_sum() {
        assert_eq!(square_of_sum(10), 3025);
    }
}
