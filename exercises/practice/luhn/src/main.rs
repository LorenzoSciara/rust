use luhn::is_valid;
use std::env::args;

fn main(){
    let args:Vec<String>=args().skip(1).collect();
    let code = args.join(" ");
    is_valid("59");
    println!("Is valid: {}", is_valid(code.as_str()));
}