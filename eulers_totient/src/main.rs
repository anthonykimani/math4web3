use crate::prime::prime::is_prime;
use crate::prime_factorization::prime_factor::get_prime_factors;

mod prime;
mod prime_factorization;

fn main() {
    find_eulers_totient(372);
}

fn find_eulers_totient(n: u64) -> u64 {
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
