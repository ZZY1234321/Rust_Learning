fn main() {
    // 首先定义一个空格字符串
    let spaces = "   "; // 这里不能用mut修饰，因为这里是定义常量，常量是不可变的
    
    // 使用变量遮蔽，重新定义spaces变量，此时它的值为原字符串的长度（空格数量），类型变为usize
    let spaces = spaces.len();
    println!("空格字符串中的空格数量为: {}", spaces);
}

// 输出：
// 空格字符串中的空格数量为: 3

/*fn main() {
    let mut spaces = "   ";
    spaces = spaces.len();
    println!("空格字符串中的空格数量为: {}", spaces);
}

报错：
(base) PS D:\CodeField\Rust\kgtj> cargo run
   Compiling kgtj v0.1.0 (D:\CodeField\Rust\kgtj)
error[E0308]: mismatched types
  --> src/main.rs:16:14
   |
15 |     let mut spaces = "   ";
   |                      ----- expected due to this value
16 |     spaces = spaces.len();
   |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `kgtj` (bin "kgtj") due to 1 previous error
*/