fn main() {
    use std::io;

    let k = 10;
    let Ans1 = assert_eq!(Some(4).unwrap_or_else(|| 2* k), 4);
    let Ans2 = assert_eq!(None.unwrap_or_else(|| 2*k), 20);
    println!("Result of Ans1 is : {:?}", Ans1);
    println!("Result of Ans2 is : {:?}", Ans2);

}
