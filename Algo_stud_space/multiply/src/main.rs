use std::io;

fn main() {
    
//    let mut input_num = String::new(); //값을 입력받기 위한 임시 변수 선언
//    io::stdin().read_line(&mut input_num).expect("Failed to read line"); //값 입력받음.
//    let mut a : i32 = input_num.trim().parse().expect("Please input interger"); // 스트링을 숫자로 변환하는 함수를 추가적용시켜야함.
//    let mut input_num = String::new(); //2번째 값을 입력받기 위한 변수 선언
//    io::stdin().read_line(&mut input_num).expect("Failed to read line"); //2번째 값 입력받음
//    let mut b : i32 = input_num.trim().parse().expect("Please input integer"); 
//    
//    let mut c = digit_value(&b); //10을 몇번 곱하는지 함수를 통해 도출.
//    let mut value_temp : Vec<i32> = Vec::new(); //나오는 값을 벡터에 삽입.
//
//    for i=0 in c {  // c 값의 횟수만큼 나눠서 a에 곱한다.
//        let value_temp[i] = &a * (b% i32::pow(10, i+1)
//        i +=1;
//        println!("{}",value_temp[i]);
//    }
//
//    let mut result = 0;
//
//    for i in value_temp[] {
//        result = result + value_temp[i] * i32::pow(10, i+1);
//    }
//
//    println!("{}",result);
    
    let mut temp_string = String::new();
    io::stdin().read_line(&mut temp_string).expect("Failed to read line");
    let mut b : i32 = temp_string.trim().parse().expect("convert string to integer is failed");
    println!("b is {}", b);
    let mut result : i32 = digit_value(b);
    println!("return value of digit_value(b) is : {}", result);

//    let test_value : i32 = i32::pow(10,result.try_into().unwrap());
//    println!("test_value is {}",test_value);
//    println!("b % test_value is {}", b % test_value);
//    println!("b / test_value is {}", b / test_value);
//
    for i in 0..result {
        println!("i is : {}",i);
    }
}


////10진수로 몇자리 수인지 알아내는 함수
pub fn digit_value ( target : i32) -> i32 {
    println!("function digit_value is started");

    let mut count_num : i32 = 0; // 10진수 체크용
    println!("function digit_value's conut_num is : {}",&count_num);

    let mut result :i32 = target; // 대상이 되는 변수의 몫 임시 저장용도
    println!("function digit value's result is : {}",&result);

    while result !=0 {
        result = result / 10; // target 을 10으로 나눈 몫 저장.
            if result !=0 {
                println!("result value is {}", result);
                count_num += 1; // 몫이 0 이 아니라면 10진수 하나 상승
                println!("count_num's value is {}",count_num);
            } else { println!("result is 0")}
    }
    count_num
}
