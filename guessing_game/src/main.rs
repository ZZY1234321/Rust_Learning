use std::io;

fn main(){
    println!("猜数游戏！");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("读取行失败");
    println!("你猜测的数是：{}", guess);

}