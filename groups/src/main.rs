fn main() {
    is_commutative([1, 2, 3, 4, 5]);
}

// need to find a way to use matrices
fn is_commutative(group: [i32; 5]) {
    let sum_a:i32 = group.iter().sum();
    let sum_b:i32 = group.iter().rev().sum();
    match sum_a == sum_b {
        true => println!("The Group is Commutative"),
        false => println!("The Group is Not Commutative"),
    }
}