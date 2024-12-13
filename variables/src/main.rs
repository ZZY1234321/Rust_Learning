fn main() {
    let mut x = 5;
    // let x = 5; 这是错误写法，因为x是可变的，不能再次赋值
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}