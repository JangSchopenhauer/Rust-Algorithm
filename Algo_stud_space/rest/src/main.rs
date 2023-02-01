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
    let mut inputString = String::new(); // 3개의 변수(a,b,c)를 하나의 줄에 다 입력받는다.
    io::stdin().read_line(&mut inputString).expect("Wrong input"); // 1줄로 입력받음
    let mut inputValue : Vec<&str> = inputString.split(' ').collect(); // std::io::stdin 의 split
                                                                       // 을 이용해서 공백을
                                                                       // 기준으로 나누고 collect
                                                                       // 로 벡터에 차례차례
                                                                       // 수집함.

    let mut a = &inputValue[0]; // 처음 받는 숫자는 a
    let mut b = &inputValue[1]; // 두번째 받는 숫자는 b
    let mut c = &inputValue[2]; // 세번째 받는 숫자는 c 가 된다.

// split 으로 나눈 벡터안의 값들이 String 인 채 이므로 u32 로 변환하는 함수를 통해 u32로 바꾼다.
// 아래에 fn convert_u32 로 구현해두었음.
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
// 전달받는 값이 스트링슬라이스 (&str) 이므로 변수 타입 지정. u32 값을 원하므로 output 은 u32 가
// 되게 선언
fn convert_u32 (input_value : &str ) -> u32 {
    //trim 메소드는 처음과 끝 부분의 빈칸을 제거한다. parse 메소드는 u32 로 변환해준다.
    let a : u32 = match input_value.trim().parse(){
        // 문제없을 경우 a로 그대로 반환
        Ok(a) => a,
        // 문제가 있을 경우 panic 을 일으키고 변환중에 일으켰으며, 나온 에러값을 출력하도록 함.
        Err(error) => { panic!("There was a problem converting the value {:?}",error)},
    };
    // 이제 변환된 값을 함수의 output 으로 보냄.
    a
}
