pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn mod_sub(a: u64, b: u64, m: u64) -> u64 {
    if a > b {
        a - b
    } else {
        m - b + a
    }
}

pub fn mod_add(a: u64, b: u64, m: u64) -> u64 {
    if b == 0 {
        a
    } else {
        mod_sub(a, m - b, m)
    }
}

pub fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    let (a, b, m) = (a as u128, b as u128, m as u128);
    ((a * b) % m) as u64
}
