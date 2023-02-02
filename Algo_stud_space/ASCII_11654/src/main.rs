use std::io;


fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Wrong input");
    let mut a:char = a.trim().parse().expect("input value is not a char");
    println!("{:?}",a as u32);

}


