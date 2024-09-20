use std::io;

fn main() {
    println!("Enter coefficients of the first polynomial (highest degree first):");
    let poly1 = get_input_vector();

    println!("Enter coefficients of the second polynomial (highest degree first):");
    let poly2 = get_input_vector();

    let sum_poly = add_polynomials(&poly1, &poly2);

    println!("The sum of the polynomials is:");
    print_polynomial(&sum_poly);
}

fn add_polynomials(poly1: &[f64], poly2: &[f64]) -> Vec<f64> {
    let max_len = poly1.len().max(poly2.len());
    let mut result = vec![0.0; max_len];

    for (i, &coeff) in poly1.iter().rev().enumerate() {
        result[max_len - 1 - i] += coeff;
    }

    for (i, &coeff) in poly2.iter().rev().enumerate() {
        result[max_len - 1 - i] += coeff;
    }

    // Remove leading zeros
    while result.len() > 1 && result[0] == 0.0 {
        result.remove(0);
    }

    result
}

fn print_polynomial(poly: &[f64]) {
    for (i, &coeff) in poly.iter().enumerate() {
        let power = poly.len() - 1 - i;
        if coeff != 0.0 {
            if i > 0 && coeff > 0.0 {
                print!(" + ");
            } else if coeff < 0.0 {
                print!(" - ");
            }
            let abs_coeff = coeff.abs();
            if abs_coeff != 1.0 || power == 0 {
                print!("{}", abs_coeff);
            }
            if power > 0 {
                print!("x");
                if power > 1 {
                    print!("^{}", power);
                }
            }
        }
    }
    println!();
}

fn get_input_vector() -> Vec<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect()
}