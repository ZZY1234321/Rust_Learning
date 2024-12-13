// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32
// }

/*
fn main() {
  // 断言0.1 + 0.2与0.3相等
  assert!(0.1 + 0.2 == 0.3);
}

报错：
   Compiling playground v0.0.1 (/playground)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s
     Running `target/debug/playground`
thread 'main' panicked at src/main.rs:3:3:
assertion failed: 0.1 + 0.2 == 0.3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

二进制精度问题，导致了 0.1 + 0.2 并不严格等于 0.3，它们可能在小数点 N 位后存在误差。如果非要进行比较，可以考虑用这种方式 (0.1_f64 + 0.2 - 0.3).abs() < 0.00001 ，具体小于多少，取决于你对精度的需求。
*/

fn main() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2);
}

// abc (f32)
//    0.1 + 0.2: 3e99999a
//          0.3: 3e99999a

// xyz (f64)
//    0.1 + 0.2: 3fd3333333333334
//          0.3: 3fd3333333333333

// thread 'main' panicked at float_wrap.rs:39:5:
// assertion failed: xyz.0 + xyz.1 == xyz.2
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace