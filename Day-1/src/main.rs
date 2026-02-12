fn main() {
    // println!("Hello, world!");
    // let a = 1;
    // let b = 3;
    // println!("a + b = {}", a + b);
    // 기본적으로 컴파일러가 자료형을 유추해 주지만, 직접 지정하는게 가능하다.
    let a = 3;
    let b: u32 = 4;
    println!("{}", a + b); // a + b 연산을 시키니 a가 i32였던게 u32로 변했다.
    // 문자열은 좀 뭔가 거시기해서 나중에 따로 다룸

}
