fn d(n: i32) -> i32 {
    let mut sum = n;
    let mut m = n;
    while m > 0 {
        sum += m % 10;
        m /= 10;
    }
    sum
}

fn main() {
    let mut is_self_number = [false; 10001];
    for i in 1..10001 {
        let mut j = d(i);
        while j < 10001 {
            is_self_number[j as usize] = true;
            j = d(j);
        }
    }
    for i in 1..10001 {
        if !is_self_number[i as usize] {
            println!("{}", i);
        }
    }
}

