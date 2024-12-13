// fn main() {
//     // 加法
//     let sum = 5 + 10;

//     // 减法
//     let difference = 95.5 - 4.3;

//     // 乘法
//     let product = 4 * 30;

//     // 除法
//     let quotient = 56.7 / 32.2;

//     // 求余
//     let remainder = 43 % 5;

//     println!("sum: {}", sum);
//     println!("difference: {}", difference);
//     println!("product: {}", product);
//     println!("quotient: {}", quotient);
//     println!("remainder: {}", remainder);
// }

// 输出：
// sum: 15
// difference: 91.2
// product: 120
// quotient: 1.7608695652173911
// remainder: 3

fn main() {
  // 编译器会进行自动推导，给予twenty i32的类型
  let twenty = 20;
  // 类型标注
  let twenty_one: i32 = 21;
  // 通过类型后缀的方式进行类型标注：22是i32类型
  let twenty_two = 22i32;

  // 只有同样类型，才能运算
  let addition = twenty + twenty_one + twenty_two;
  println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

  // 对于较长的数字，可以用_进行分割，提升可读性
  let one_million: i64 = 1_000_000;
  println!("{}", one_million.pow(2));

  // 定义一个f32数组，其中42.0会自动被推导为f32类型
  let forty_twos = [
    42.0,
    42f32,
    42.0_f32,
  ];

  // 打印数组中第一个值，并控制小数位为2位
  println!("{:.2}", forty_twos[0]);
  println!("{:.2}", forty_twos[1]);
    println!("{:.2}", forty_twos[2]);
}

// 输出：
// 20 + 21 + 22 = 63
// 1000000000000
// 42.00
// 42.00
// 42.00