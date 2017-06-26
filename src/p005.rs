/// Problem 5: Smallest multiple

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

/// smallest multiple of numbers 1..n
fn smallest_multiple(n: u64) -> u64 {
    (2..n + 1).fold(1, lcm)
}

pub fn solve() -> u64 {
    smallest_multiple(20)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(2, 6), 2);
        assert_eq!(gcd(6, 9), 3);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(2, 6), 6);
        assert_eq!(lcm(6, 9), 18);
    }

    #[test]
    fn test_smallest_multiple() {
        assert_eq!(smallest_multiple(10), 2520);
    }
}
