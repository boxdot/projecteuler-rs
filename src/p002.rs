struct Fib {
    curr: u64,
    next: u64,
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fib() -> Fib {
    Fib { curr: 1, next: 2 }
}

fn sum_of_even_fib(limit: u64) -> u64 {
    fib()
        .take_while(|&x| x < limit)
        .filter(|x| x % 2 == 0)
        .sum()
}

pub fn solve() -> u64 {
    sum_of_even_fib(4_000_000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_even_fib() {
        assert!(sum_of_even_fib(90) == 2 + 8 + 34);
    }
}
