// rustc docs/number.rs && ./number

fn main() {
    let a: i32 = 33;
    println!("{}", a + 1);

    let b = 33i32;
    println!("{}", b + 1);

    let c = 33.569999f64;
    println!("{}", c + 1 as f64);

    // 四舍五入
    let d = format!("{:.2}", c);
    println!("{}", d);
    println!("{:.2}", c);

    // 向下取整
    println!("{}", c.floor());

    // 向上取整
    println!("{}", c.ceil());
}
