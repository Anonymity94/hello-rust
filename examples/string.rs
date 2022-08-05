// rustc docs/string.rs && ./string

// @see https://rustwiki.org/zh-CN/rust-by-example/std/str.html
// @see https://rust-book.junmajinlong.com/ch3/04_str_string.html

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

    // &str => string
    let str1_string = str1.to_string();
    println!("&str => string: {}", str1_string);

    // 定义 string
    let mut string = String::from("Hello");
    // 拼接单个 char
    string.push('.');
    // 拼接 &str
    string.push_str("Word");
    println!("{}", string);

    // 字符串替换
    let alice = String::from("I like dogs");
    let alice2 = alice.replace("dog", "cat");
    println!("{}", alice2);

    // 字符串反转
    println!("{}", alice.chars().rev().collect::<String>());

    // 字符串截取
    // 利用字符串分片
    let s1 = &alice[0..5];
    println!("{}", s1);
    let s2 = &alice[5..alice.len()];
    println!("{}", s2);

    // 字符串去重
    let pangram: &str = "the quick brown fox jumps over the lazy dog";
    let mut chars: Vec<char> = pangram.chars().collect();
    // 排序
    chars.sort();
    // 去重
    chars.dedup();
    println!("{}", chars.iter().collect::<String>());
}
