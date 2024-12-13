fn main() {
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    println!("a = {:?}, b = {:?}", a, b);
    assert_eq!(a, b); // 断言，如果a等于b，程序继续执行，否则程序panic并输出信息
    b = false;
    println!("a = {:?}, b = {:?}", a, b);
    assert_ne!(a, b); // 断言，如果a不等于b，程序继续执行，否则程序panic并输出信息
    b = true;
    println!("a = {:?}, b = {:?}", a, b);
    assert_ne!(a, b); // 断言，如果a不等于b，程序继续执行，否则程序panic并输出信息
}