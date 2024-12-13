fn main() {
    /* 错误示例：
    let guess = "42".parse().expect("Not a number!");
    以下是报错信息：
    error[E0284]: type annotations needed
     --> src/main.rs:3:9
      |
    3 |     let guess = "42".parse().expect("Not a number!");
      |         ^^^^^        ----- type must be known at this point
      |
      = note: cannot satisfy `<_ as FromStr>::Err == _`
    help: consider giving `guess` an explicit type
      |
    3 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
      |              ++++++++++++

    For more information about this error, try `rustc --explain E0284`.
    error: could not compile `rust_parse_demo` (bin "rust_parse_demo") due to 1 previous error
    这段代码的目的是将字符串 "42" 进行解析，而编译器在这里无法推导出我们想要的类型：整数？浮点数？字符串？因此编译器会报错。
*/

    // 正确示例：
    // 方式一：使用类型标注的方式明确变量类型
    let guess: i32 = "42".parse::<i32>().expect("Not a number!");
    println!("解析后的数字为: {}", guess);

    // 方式二：先将解析结果放入Result类型中，再进行处理
    let result: Result<i32, _> = "42".parse();
    match result {
        Ok(num) => println!("通过匹配处理，解析后的数字为: {}", num),
        Err(e) => println!("解析出错，错误信息: {}", e),
    }
}