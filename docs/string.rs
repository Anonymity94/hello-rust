fn main() {
    // char 类型，必须用单引号包裹
    let char = '我';
    println!("{}是 char 类型", char);

    // 定义 string
    let mut string = String::from("Hello");
    // 拼接单个 char
    string.push('.');
    // 拼接 &str
    string.push_str("Word");
    println!("{}", string);
}
