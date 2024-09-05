fn extended_gcd(a:i64, b:i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a/ b) * y)
    }
}

pub fn mod_inverse(a: i64, m: i64) -> Result<i64, &'static str> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        return Err("No modular inverse found, e and phi are not coprime.");
    } else {
        Ok((x % m + m) % m)
    }
}


