use num::complex::Complex;
// complex-num 是一个库，它提供了一个复数类型 Complex<T>，其中 T 是实数类型。这个库提供了一些方法，比如 add 和 new，用于复数的加法和创建。
// 通过 use num::complex::Complex; 引入了 Complex 类型。

 fn main() {
   // 创建两个复数a和b
   // 注意：Complex::new() 方法需要两个实数参数，分别表示实部和虚部。rm 和 im 是 Complex 结构体的字段。re 表示实部，im 表示虚部。
   // 这里创建了复数 a 和 b，它们的实部分别为 2.1 和 11.1，虚部分别为 -1.2 和 22.2。
   // 即 a = 2.1 - 1.2i，b = 11.1 + 22.2i。
   let a = Complex { re: 2.1, im: -1.2 };
   let b = Complex::new(11.1, 22.2);
   let result = a + b;

   // 输出时要实部虚部分开输出，注意：result.re 和 result.im 分别表示 result 的实部和虚部。
   println!("{} + {}i", result.re, result.im)
 }

//  输出：
//  13.2 + 21i