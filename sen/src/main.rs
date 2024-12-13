// fn add_with_extra(x: i32, y: i32) -> i32 {
//     let x = x + 1; // 语句
//     let y = y + 5; // 语句
//     x + y // 表达式
// }

fn main(){
    let a = 8;
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false);
    // 以上都是语句，它们完成了一个具体的操作，但是并没有返回值，因此是语句。

    // 由于 let 是语句，因此不能将 let 语句赋值给其它值，如下形式是错误的：
    // let d = (let d = 8);
//     error: expected expression, found `let` statement
//   --> src/main.rs:14:14
//    |
// 14 |     let d = (let d = 8);
//    |              ^^^
//    |
//    = note: only supported directly in conditions of `if` and `while` expressions

}