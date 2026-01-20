use std::io; // 입출력 표준 라이브러리

fn main() {
    println!("Guess a Number!!"); // 출력 매크로

    let mut input = String::new(); // 변수 선언.

    // 러스트는 기본적으로 변수가 불변임.
    // let apple = 3; << 나중에 수정 불가.
    // 가변 변수로 선언하려면 mut 키워드 사용.
    // let mut input = String::new(); 는 가변하는 빈 string 인스턴스를 변수에 넣은것임.

    io::stdin()
        .read_line(&mut input)
        .expect("Filed to read line");
    // io::stdin() io 라이브러리에서 stdin 함수로 Stdin의 인스턴스 획득
    // 표준 입력 핸들에서 read_line 메서드 호출
    // &mut input : input 변수에 대한 가변 참조자임을 전달
    // & 사용하는 이유는 레퍼런스 사용 때문.
    // read_line 메서드는 Result  enum 타입을 반환함. 여러가지 상태를 가지기에 variant 라고도 함.
    // result 타입은 Ok, Err 두가지 variant 가 있음.
    // expect 메서드는 Result 타입에서 Err variant 가 반환될 경우 프로그램을 패닉 시키고 메시지를 출력함.
    // expect 는 사용하지 않아도되지만, 컴파일시 워닝을 던짐.

    

    


    println!("Your Pick : {input}");
}
