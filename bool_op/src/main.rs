fn main() {
    let t = true;

    let f: bool = false; // 使用类型标注,显式指定f的类型
    let mut g = false;
    g = true;

    if f {
        println!("这是段毫无意义的代码");
    }
    if g {
        println!("这是真的代码");
    }
}