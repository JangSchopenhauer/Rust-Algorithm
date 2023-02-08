use std::io;

fn main() {

    //듣지못한사람 N, 보지못한사람 M 을 입력받는다.
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("wrong input");
    let mut input : u32 = input.trim().parse().expect("It is not u32 type");

    //입력받은 값 2개를 ' ' 으로 split 한 후 저장.


    //N, M 의 횟수만큼 read_line

    //모든 듣지못한사람과 보지못한사람을 비교하여 동일한 이름이 나오면 다른 벡터에 저장

    //해당 벡터의 첫글자를 ASCII 코드와 비교하여 작은애가 앞으로 오도록 하는 함수 구현.

    //듣지도 보지도 못한 사람 숫자 + 이름 출력
}
