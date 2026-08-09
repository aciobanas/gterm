/// Euclidean algorithm; runs in `O(log(min(a, b)))` and matches the convention `gcd(0, x) == x`.
pub(crate) const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
