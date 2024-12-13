fn main() {
    let x = 5;
    // 如果有一个变量无需用到，但是这个变量的创建又是有意义的，可以在变量名前加上下划线，这样编译器就不会报错
    let _y = 10;
    println!("The value of x is: {}", x);
}