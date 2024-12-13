fn main() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    let x = '中';
    let dog = '🐶';
    let shit = '💩';

    println!("c is {}", c);
    println!("z is {}", z);
    println!("g is {}", g);
    println!("heart_eyed_cat is {}", heart_eyed_cat);
    println!("dog is {}", dog);
    println!("shit is {}", shit);
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
}

// 输出：
// c is z
// z is ℤ
// g is 国
// heart_eyed_cat is 😻
// dog is 🐶
// shit is 💩
// 字符'中'占用了4字节的内存大小