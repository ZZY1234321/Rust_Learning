// fn plus_five(x:i32) -> i32 {
//     x + 5
// }

// fn main() {
//     let x = plus_five(5);

//     println!("The value of x is: {}", x);
// }

// 输出：
// The value of x is: 10

fn plus_or_minus(x:i32) -> i32 {
    if x > 5 {
        return x - 5
    }

    x + 5
}

fn main() {
    let x = plus_or_minus(5);

    println!("The value of x is: {}", x);
}

// 输出：
// 参数是5时：The value of x is: 10
// 参数是10时：The value of x is: 5

