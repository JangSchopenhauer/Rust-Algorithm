use std::io;


fn main() {
    println!("Input Char to convert ASCII Code")
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Wrong input");
    let mut a = a.trim().parse::<u32>().expect("input value is not a char");
    println!("Input char is {:?} and it's ASCII Code is {:?}",a, a as u32);

}


