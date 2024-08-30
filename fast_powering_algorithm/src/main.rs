fn main() {
    let base = 3 ;
    let exponent = 265;
    let modulus = 17;
    let result = mod_exp(base, exponent, modulus);
    println!("{}^{} mod {} = {}", base, exponent,  modulus, result);
}
fn mod_exp(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    result
}