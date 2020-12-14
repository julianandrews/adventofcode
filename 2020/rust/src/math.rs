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

// Shamelessly stolen from
// https://stackoverflow.com/questions/45918104/how-to-do-arithmetic-modulo-another-number-without-overflow
pub fn mod_mul(mut a: u64, mut b: u64, m: u64) -> u64 {
    let msb = 0x8000_0000_0000_0000;
    let mut d = 0;
    let mp2 = m >> 1;
    a %= m;
    b %= m;

    if m & msb == 0 {
        for _ in 0..64 {
            d = if d > mp2 { (d << 1) - m } else { d << 1 };
            if a & msb != 0 {
                d += b;
            }
            if d >= m {
                d -= m;
            }
            a <<= 1;
        }
        d
    } else {
        for _ in 0..64 {
            d = if d > mp2 {
                d.wrapping_shl(1).wrapping_sub(m)
            } else {
                // the case d == m && a == 0 is taken care of
                // after the end of the loop
                d << 1
            };
            if a & msb != 0 {
                let (mut d1, overflow) = d.overflowing_add(b);
                if overflow {
                    d1 = d1.wrapping_sub(m);
                }
                d = if d1 >= m { d1 - m } else { d1 };
            }
            a <<= 1;
        }
        if d >= m {
            d - m
        } else {
            d
        }
    }
}
