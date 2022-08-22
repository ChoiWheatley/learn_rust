/// 소유권 규칙
/// 1. 모든 값은 owner라고 불리우는 변수가 가지고 있다.
/// 2. 모든 값은 한 번에 딱 하나의 owner만 존재할 수 있다. (unique_ptr???)
/// 3. owner가 스코프 밖으로 벗어나는 때 값은 버려진다. (dropped)
pub fn make_memory_move() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2는 정상동작함: {}", s2);
    // println!("s1은? 소유권 뺐겨서 컴파일 에러 발생. {}", s1);
}
