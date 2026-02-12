use std::io;

fn main() {
    // println!("Hello, world!");
    // let a = 1;
    // let b = 3;
    // println!("a + b = {}", a + b);
    // // 기본적으로 컴파일러가 자료형을 유추해 주지만, 직접 지정하는게 가능하다.
    // let c = 3;
    // let d: u32 = 4;
    // println!("{}", c + d); // a + b 연산을 시키니 a가 i32였던게 u32로 변했다.
    // // 문자열은 좀 뭔가 거시기해서 나중에 따로 다룸
    // let e = 1; // mut 키워드를 지정하지 않으면 값을 변경할 수 없다.
    // // e = 3; 에러 발생
    // let mut f = 3; // 이번엔 mut 키워드를 지정함
    // f = 4; // 에러가 발생하지 않음
    // println!("{}", f);

    let mut inp = String::new(); // 문자열 생성 (문자열 자세히는 몰루)
    io::stdin().read_line(&mut inp).expect("Failed to read line"); // 아마 대충 inp를 참조로 넘겨주고 거기다가 값을 대입하는거 같음
    println!("{}", inp.trim())

}
