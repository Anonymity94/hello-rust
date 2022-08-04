// rustc docs/string.rs && ./string

fn main() {
    // char 类型，必须用单引号包裹
    let char = '我';
    println!("{}是 char 类型", char);

    // &str 类型
    // 其中str表示str类型，&表示该类型的引用，即一个指针
    let str1 = "你好啊派大星。";
    // 等价于
    let str2: &str = "你好啊派大星。2";
    println!("{},{}", str1, str2);

    // 定义 string
    let mut string = String::from("Hello");
    // 拼接单个 char
    string.push('.');
    // 拼接 &str
    string.push_str("Word");
    println!("{}", string);
}
