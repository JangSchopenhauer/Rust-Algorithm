use std::io;
use std::io::stdin;


fn main() {
//    println!("please input value a ");
//    let mut input = String::new();
//    io::stdin().read_line(&mut input).unwrap();
//    let a : i32 = input.trim().parse().unwrap();
//    println!("value of a is {}",a);

//    let a : i32 = get_value();
    println!("write value a, b, c. separate each as Enter");
    let mut inputString = String::new();
    io::stdin().read_line(&mut inputString).expect("Wrong input");
    let mut inputValue : Vec<&str> = inputString.split(' ').collect();

    let mut a = &inputValue[0];
    let mut b = &inputValue[1];
    let mut c = &inputValue[2];

    let mut a = convert_u32(&a);
    let mut b = convert_u32(&b);
    let mut c = convert_u32(&c);

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

fn convert_u32 (input_value : &str ) -> u32 {
    let a : u32 = match input_value.trim().parse(){
        Ok(a) => a,
        Err(error) => { panic!("There was a problem converting the value {:?}",error)},
    };
    a
}
