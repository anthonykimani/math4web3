use std::any::type_name;
fn main() {
    get_binary(234);
}

fn get_binary(number: i64) -> Vec<i64> {
    let mut halve = number / 2;
    let mut binary = vec![];

    while halve != 0 {
        if type_of(halve) ==  {
            binary.push(0);
        } else if type_of(halve) ==  {
            binary.push(1);
        } else {
            println!("Type not integer or float");
        }
        halve / 2;
    }
    println!("The Binary of {} is {:?}",number, binary);
    return binary;
}



fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}