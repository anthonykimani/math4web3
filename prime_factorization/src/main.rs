fn main() {
    let number = 56;
    let factors = get_prime_factors(number);
    println!("The Prime Factors of {} is {:?}", number, factors);
}

fn get_prime_factors(mut number:i32) -> Vec<i32> {
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