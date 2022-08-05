// rustc docs/statement-expression.rs && ./statement-expression

// @see https://course.rs/basic/base-type/statement-expression.html
// 表达式总有返回值

fn main() {
    let y = {
        let x = 3;
        // 这里不能添加 ;
        // 不添加分号，表示是个表达式，表达式会有返回值
        // 添加了分号，表示是个语句
        x + 1
    };

    println!("The value of y is: {}", y);
}
