fn main() {
    let points = vec![(1, 2), (3, 6), (5, 10)];
    let x = 5;

    let result = lagrange_interpolation(points, x);
    println!("Interpolated value at x = {} is {}", x, result);
    
}

fn lagrange_interpolation(points: Vec<(i32,i32)>, x: i32) -> f64 {
    let mut result = 0.0;

    for ( x_i, y_i ) in &points {
        let mut l_i = 1.0;

        for ( x_j, _ ) in &points {
            if x_i != x_j {
                l_i *= (x as f64 - *x_j as f64) / (*x_i as f64 - *x_j as f64);
            }
        }

        result += *y_i as f64 * l_i;
    }

    result
}
