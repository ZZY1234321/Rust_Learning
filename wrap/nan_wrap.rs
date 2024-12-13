// 对负数取平方根 -42.1.sqrt() ，会产生一个特殊的结果：Rust 的浮点数类型使用NaN来处理这些情况。

// fn main() {
//   let x = (-42.0_f32).sqrt();
//   assert_eq!(x, x);
// }

// 所有跟 NaN 交互的操作，都会返回一个 NaN，而且 NaN 不能用来比较，此代码会崩溃。

// thread 'main' panicked at src/main.rs:3:3:
// assertion `left == right` failed
//   left: NaN
//  right: NaN
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

fn main() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }
}

// 输出：未定义的数学行为