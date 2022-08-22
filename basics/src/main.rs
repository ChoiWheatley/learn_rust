fn main() {
    expressions();
    iteration();
}

fn expressions() {
    // 구문(statement) != 표현식(expression)
    // 구문은 값을 리턴하지 않는다.
    // 표현식은 값을 리턴한다.
    // 기본적으로 `let` 키워드는 구문으로, 값을 리턴하지 않는다.
    let _y = 6;
    // let x = (let y = 6); // 컴파일 에러.
    let _x = 5;
    let _y = {   // block `{}`은 표현식.
        let _x = 3;
        _x + 1   // 세미콜론 `;`가 없는 것을 유념.
                // 세미콜론이 붙으면 구문, 없으면 표현식. 
    };
    println!("{_y}");

    // 반환값을 갖는 함수 사용
    println!("five is {}.", five());

    // if문은 기본적으로 표현식.
    // if 내부가 표현식이라면 if문 자체가 값을 리턴할 수 있다.
    let condition = true;
    let number = if condition { 5 } else { 6 }; // 표현식의 경우에만 `;`
    println!("number is {}", number);

}

/// 반환값을 갖는 함수
fn five() -> i32 {
    5   // 세미콜론이 없으면 표현식. 값을 반환한다.
        // 즉, `return 5`와 동치이다.
}

fn iteration() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    };
}
