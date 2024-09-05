

pub fn find_eulers_totient(n: u64) -> u64 {
    let mut prime_factors: Vec<u64>= Vec::new();
    let mut factors: Vec<u64>= get_prime_factors(n).into_iter().filter(|&x| {
        if prime_factors.contains(&x) {
            false
        } else {
            prime_factors.push(x);
            true
        }
    }).collect();

    if is_prime(n) {
        println!("Euler's Totient is {:?}", n - 1);
        n - 1
    } else if factors.len() == get_prime_factors(n).len(){
        let mut totient: u64 = 1;
        for element in prime_factors {
            totient *= element - 1;
        }
        println!("Euler's Totient is {}", totient);
        totient
    } else {
        let mut result = n as f64;
        for element in prime_factors {
            result *= 1.0 - (1.0 / element as f64);
        }
        println!("Euler's Totient is {}", result as u64);
        result as u64
    }
}



pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
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