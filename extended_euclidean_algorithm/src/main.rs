fn extended_gcd(mut dividend: i64, mut divisor: i64) -> (i64, i64, i64) {
    let mut old_coefficient_dividend = 0;
    let mut old_coefficient_divisor = 1;
    let mut coefficient_dividend = 1;
    let mut coefficient_divisor = 0;

    while divisor != 0 {
        let quotient = dividend / divisor;
        let remainder = dividend % divisor;
        let new_coefficient_dividend = old_coefficient_dividend - coefficient_dividend * quotient;
        let new_coefficient_divisor = old_coefficient_divisor - coefficient_divisor * quotient;

        dividend = divisor;
        divisor = remainder;
        old_coefficient_dividend = coefficient_dividend;
        old_coefficient_divisor = coefficient_divisor;
        coefficient_dividend = new_coefficient_dividend;
        coefficient_divisor = new_coefficient_divisor;
    }

    (dividend, old_coefficient_dividend, old_coefficient_divisor)
}

fn main() {
    let (number1, number2) = (48, 18);
    let (gcd, coefficient1, coefficient2) = extended_gcd(number1, number2);
    println!("GCD of {} and {} is {}", number1, number2, gcd);
    println!("Bezout's identity coefficients: x = {}, y = {}", coefficient1, coefficient2);
    println!("Verification: {}*{} + {}*{} = {}", number1, coefficient1, number2, coefficient2, gcd);
}