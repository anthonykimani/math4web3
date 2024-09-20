use std::io;

fn main() {
    println!("Enter the coefficients of the first polynomial (with the highest degree first):");
    let first_polynomial_coefficients = get_input_vector();

    println!("Enter the coefficients of the second polynomial (with the highest degree first):");
    let second_polynomial_coefficients = get_input_vector();

    for ( power, &first_coeff ) in first_polynomial_coefficients.iter().rev().enumerate() {
        for ( power, &second_coeff ) in second_polynomial_coefficients.iter().rev().enumerate() {
            let mut var = &first_coeff + &second_coeff;
            println!("{:?}",var);
        }
    }
}

fn get_input_vector() -> Vec<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid Number"))
        .collect()
}
