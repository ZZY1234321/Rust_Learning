fn main() {
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效
    // println!("在move进函数后继续使用s: {}",s);
    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

// 若不删去第五行，报错信息如下：
/*
error[E0382]: borrow of moved value: `s`
  --> src/main.rs:5:35
   |
2  |     let s = String::from("hello");  // s 进入作用域
   |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3  |     takes_ownership(s);             // s 的值移动到函数里 ...
   |                     - value moved here
4  |                                     // ... 所以到这里不再有效
5  |     println!("在move进函数后继续使用s: {}",s);
   |                                            ^ value borrowed here after move
   |
note: consider changing this parameter type in function `takes_ownership` to borrow instead if owning the value isn't necessary
  --> src/main.rs:11:33
   |
11 | fn takes_ownership(some_string: String) { // some_string 进入作用域
   |    ---------------              ^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
3  |     takes_ownership(s.clone());             // s 的值移动到函数里 ...
   |                      ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `transfer_value` (bin "transfer_value") due to 1 previous error
*/