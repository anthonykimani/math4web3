# Modular Arithmetic

Modular arithmetic is a system of arithmetic for integers, where numbers "wrap around" upon reaching a certain value—the modulus. It’s like the arithmetic you might do with a clock: once you go past 12, you start again from 1.

### Key concepts
1. Modulus - The number at which the value resets to zero, in a clock the modulus is 12.
2. Congruence - Two numbers are said to be congruent modulo a number, if they give the same remainder when divided by a modulus
    - For example 17 ≡ 5 mod 12

#### Applications

- Cryptography: Modular arithmetic underpins many cryptographic algorithms, such as RSA encryption.
- Computer Science: It is used in algorithms, hashing, and in creating cyclic data structures.
- Number Theory: It helps in solving problems related to divisibility, prime numbers, and Diophantine equations.

#### Rust Implementation
```rust
fn main() {
    find_mod();
}

fn find_mod() -> i32 {
    println!("Enter Number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let rem: i32 = input.trim().parse().expect("Please enter a number");
    println!("Enter Modulus: ");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let modulus: i32 = input2.trim().parse().expect("Please enter a number");
    if rem >= modulus {
        let ans = rem % modulus ;
        println!("{} mod {} = {}", rem, modulus, ans);
        ans
    } else {
        println!("{} mod {} = {}", rem, modulus, rem);
        rem
    }
}
```


This program accepts two inputs, a number and a modulus and outputs the remainder