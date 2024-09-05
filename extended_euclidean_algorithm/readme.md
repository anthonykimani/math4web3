# Extended Euclidean Algorithm in Rust

This Rust program calculates the Greatest Common Divisor (GCD) of two numbers using the Euclidean algorithm.

## How It Works


The Extended Euclidean Algorithm is an extension of the Euclidean Algorithm, which is used to find the Greatest Common Divisor (GCD) of two integers. The Extended Euclidean Algorithm not only finds the GCD of two integers but also provides coefficients (often called Bézout coefficients) that can express the GCD as a linear combination of the two integers. Specifically, it finds integers
𝑥
x and
𝑦
y such that:

```markdown
GCD(a, b) = ax + by
```

## What Does the Extended Euclidean Algorithm Allow Us to Do?

1. **Compute Bézout's Identity**: The algorithm provides the integers \( x \) and \( y \) such that \( ax + by = GCD(a, b) \). This is useful in number theory and cryptography.

2. **Find Multiplicative Inverses**: If \( a \) and \( b \) are coprime (i.e., \( GCD(a, b) = 1 \)), the algorithm can find the multiplicative inverse of \( a \) modulo \( b \). This is because \( ax ≡ 1  (mod b) \) implies that \( x \) is the multiplicative inverse of \( a \) modulo \( b \).

3. **Solve Linear Diophantine Equations**: The Extended Euclidean Algorithm can solve equations of the form \( ax + by = c \), where \( c \) is a multiple of the GCD of \( a \) and \( b \). If \( c \) is not a multiple of the GCD, no solution exists.

4. **Used in Cryptography**: It is fundamental in algorithms like RSA for key generation and decryption, where finding modular inverses is essential.

#### For Example
![img_1.png](img_1.png)

1. We will first find the GCD using the Euclidean algorithm
2. Then since 1 is the GCD, we apply the Extended Euclid Algorithm by making 1 in the second last equation the subject of the formula. Then substituting all the remainders one at a time going up.

![img_2.png](img_2.png)

## Example Rust Implementation

```rust
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
```