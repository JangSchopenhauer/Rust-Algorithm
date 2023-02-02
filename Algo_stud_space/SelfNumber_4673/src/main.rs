fn main() {

    let mut a = 1;
    println!("{:?}",a);
    for a in 1..10 {
        let mut a = a + 2;
        println!("{:?}",a);
    };
    for a in 10..100 {
        let mut a = a + 11;
        println!("{:?}",a);
    };
    for a in 100..1000 {
        let mut a = a+101;
        println!("{:?}",a);
    };
    for a in 1000..10000 {
        let mut a = a+1001;
        println!("{:?}",a);
    };
}

