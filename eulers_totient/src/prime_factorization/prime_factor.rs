
pub fn get_prime_factors(mut number:u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut divisor = 2;

    while number >= divisor * divisor {
        while number % divisor == 0 {
            factors.push(divisor);
            number = number / divisor;
        }
        divisor += 1;
    }

    if number > 1 {
        factors.push(number);
    }

    factors
}