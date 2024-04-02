/// Largest palindrome product of two 3-digit numbers

fn num_digits(n: u32) -> u32 {
    let mut n = n;
    let mut num = 0;
    while n > 0 {
        n /= 10;
        num += 1;
    }
    num
}

fn ith_digit(n: u32, i: u32) -> u32 {
    let mut n = n;
    for _ in 0..i {
        n /= 10;
    }
    n % 10
}

fn is_palindromic_number(n: u32) -> bool {
    let num = num_digits(n);
    for i in 0..(num / 2) {
        if ith_digit(n, i) != ith_digit(n, num - i - 1) {
            return false;
        }
    }
    true
}

fn largest_palindrome_product(num_digits_in_factor: u32) -> u32 {
    let from = 10u32.pow(num_digits_in_factor - 1);
    let to = 10u32.pow(num_digits_in_factor) - 1;

    let mut palindrom = 0;
    for x in (from..to + 1).rev() {
        for y in (from..x + 1).rev() {
            let prod = x * y;
            if is_palindromic_number(prod) && prod > palindrom {
                palindrom = prod;
            }
        }
    }
    palindrom
}

pub fn solve() -> u32 {
    largest_palindrome_product(3)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_palindromic_number() {
        assert!(is_palindromic_number(9009));
        assert!(is_palindromic_number(90009));
        assert!(is_palindromic_number(900009));
    }

    #[test]
    fn test_largest_palindrome_product() {
        assert_eq!(largest_palindrome_product(2), 9009);
    }
}
