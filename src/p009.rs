/// Problem 9: Special Pythagorean triplet
///
/// Generating formula:
///
/// a = m^2 - n^2, b = 2mn, c = m^2 + n^2,
///
/// for m > n > 0.
///
/// 1000 = a + b + c = m^2 - n^2 + 2mn + m^2 + n^2 = 2m(m + n)
///  <=>  500 = m(m + n)
///
/// From m > n > 0 follows:
///   m = 20, n = 5
///
/// => a = 375, b = 200, c = 425

pub fn solve() -> u64 {
    // cf. above computation
    23375000
}
