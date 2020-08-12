// 使用 标准库(std) io
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜数字游戏!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("请输入你猜的数字");



        // 变量声明
        // 使用 mut 标识 可变
        // let foo = 5; // 不可变
        // let mut bar = 5; // 可变
        let mut guess = String::new();

        // 标准输入
        // read_line 的工作是，无论用户在标准输入中键入什么内容，都将其存入一个字符串中，因此它需要字符串作为参数。
        // 这个字符串参数应该是可变的，以便 read_line 将用户输入附加上去。
        // & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。
        // 引用是一个复杂的特性，Rust 的一个主要优势就是安全而简单的操纵引用。
        // 完成当前程序并不需要了解如此多细节。现在，我们只需知道它像变量一样，默认是不可变的。因此，需要写成 &mut guess 来使其可变，而不是 &guess。
        io::stdin().read_line(&mut guess)
            .expect("读取输入失败");

        // {} 打印 占位符
        println!("你猜的数字是: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("恭喜你答对了");
                break;
            },
        }
    }

}
