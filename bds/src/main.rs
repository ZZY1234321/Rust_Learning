fn main() {
    let y = {
        let x = 3;
        x + 1 // 注意 x + 1 不能以分号结尾，否则就会从表达式变成语句，表达式不能包含分号
    };

    println!("The value of y is: {}", y);
}

// 输出：
// The value of y is: 4

// fn main() {
//     assert_eq!(ret_unit_type(), ())
// }

// fn ret_unit_type() {
//     let x = 1;
//     // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
//     // 类似三元运算符，在Rust里我们可以这样写
//     let y = if x % 2 == 1 {
//         "odd"
//     } else {
//         "even"
//     };
//     // 或者写成一行
//     let z = if x % 2 == 1 { "odd" } else { "even" };
// }
// 啥也不输出，因为函数没有返回值