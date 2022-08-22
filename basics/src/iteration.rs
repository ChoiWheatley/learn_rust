pub fn iteration() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    println!("느리고 위험한 방법. 인덱스 직접 참조하기");
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    };
    
    println!("이터레이터를 사용하는 방식");
    for element in a.iter() {
        println!("the value is {}", element);
    }

    println!("역순 이터레이터도 있어요.");
    for element in a.iter().rev() {
        println!("the value is {}", element);
    }

    println!("Range는 한 숫자에서 다른 숫자 전까지 모든 숫자를 차례로 생성.");
    for number in (1..8).rev() {
        println!("the value is {}", number);
    }
}

