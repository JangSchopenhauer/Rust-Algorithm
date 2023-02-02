fn main() {
    let mut numbers = [true; 10001];
    for i in 1..10000 {
        let mut n = i;
        let mut m = i;
        while n > 0 {
            m += n % 10;
            n /= 10;
        }
        if m <= 10000 {
            numbers[m as usize] = false;
        }
    }
    for i in 1..10000 {
        if numbers[i as usize] {
            println!("{} is a self number", i);
        }
    }
}

