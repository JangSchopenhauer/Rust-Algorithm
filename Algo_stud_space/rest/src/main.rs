use std::io;


fn main() {
//    println!("please input value a ");
//    let mut input = String::new();
//    io::stdin().read_line(&mut input).unwrap();
//    let a : i32 = input.trim().parse().unwrap();
//    println!("value of a is {}",a);

//    let a : i32 = get_value();
    println!("write value a, b, c. separate each as Enter");
    let a = get_value().trim().parse::<i32>().unwrap();
    let b = get_value().trim().parse::<i32>().unwrap();
    let c = get_value().trim().parse::<i32>().unwrap();
    println!("value a is : {}",a);
    println!("value b is : {}",b);
    println!("value c is : {}",c);

//    println!("result of (a+b)%c : {}", (a+b)%c);
    println!("case 1 result is ");
    if (&a+&b)%&c == ((&a%&c)+(&b%&c))%&c {
        println!("true");
    } else {println!("False");}


}

//fn get_value() -> i32 {
//    let mut inpv = String::new();
//    io::stdin().read_line(&mut inpv).expect("wrong value");
//    let mut outpv : i32 = inpv.parse::<F>().unwrap().expect("error occured in converting process");
//    outpv
//}

fn get_value() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

