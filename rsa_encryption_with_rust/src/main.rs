mod totient;
mod extended_euclid;

use totient::find_eulers_totient;
use extended_euclid::mod_inverse;

fn main() {
    generate_rsa_key();
}


fn generate_rsa_key()  {
    println!("Enter First Prime Number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_prime: u64 = input.trim().parse().expect("Please enter a prime number");

    println!("Enter Second Prime Number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let second_prime: u64 = input.trim().parse().expect("Please enter a prime number");

    let modulus = first_prime  * second_prime;
    let totient = find_eulers_totient(modulus);
    println!("{totient}");

    println!("Choose RSA Public Key: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let rsa_public_key: i64 = input.trim().parse().expect("Please enter a prime number");


    match mod_inverse( rsa_public_key, totient as i64) {
        Ok(inv) => {
            println!("The modular inverse of {} modulo {} is {}", rsa_public_key , totient, inv);
        },
        Err(err) => eprintln!("Error: {}", err),
    }
}