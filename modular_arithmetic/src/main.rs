fn main() {
    find_mod();
}

fn find_mod() -> i32 {
    println!("Enter Remainder: ");
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