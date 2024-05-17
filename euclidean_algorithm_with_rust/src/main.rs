fn main() {
    get_gcd(12,28);
}

fn get_gcd(num_one: i32, num_two: i32) {
    let mut gcd_a = num_one;
    let mut gcd_b = num_two;

    if gcd_b > gcd_a {
        let mut divider = (gcd_b / gcd_a) as i32;
        let mut remainder = gcd_b - (gcd_a * divider);
        while remainder != 0 {
            gcd_b = gcd_a;
            gcd_a = remainder;
            divider = (gcd_b / gcd_a) as i32;
            remainder = gcd_b - (gcd_a * divider);
        }
        println!(" The GCD of {} and {} is {}", num_one, num_two, gcd_a);
    }

    if gcd_b < gcd_a {
        let mut divider = (gcd_a / gcd_b) as i32;
        let mut remainder = gcd_a - (gcd_b * divider);
        while remainder != 0 {
            gcd_a = gcd_b;
            gcd_b = remainder;
            divider = (gcd_a / gcd_b) as i32;
            remainder = gcd_a - (gcd_b * divider);
        }
        println!(" The GCD of {} and {} is {}", num_one, num_two, gcd_b);
    }
}
