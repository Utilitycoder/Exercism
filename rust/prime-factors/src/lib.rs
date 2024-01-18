pub fn factors(n: u64) -> Vec<u64> {
    let mut f = n;
    let mut factors = Vec::new();
    let mut d = 2;
    while f > 1 {
        while f % d == 0 {
            factors.push(d);
            f /= d;
        }
        d += 1;
    }
    factors
}
