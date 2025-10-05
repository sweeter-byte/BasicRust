use std::io; // 输入/输出库

use rand::Rng; // rand crate. Rng 是一个 trait

use std::cmp::Ordering; // enum: Less, Greater, Equal


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // 上下边界均为闭区间

    //println!("The secret number is: {secret_number}");

    loop{
        println!("Please input your guess.");

        let mut guess = String::new(); // 可变。 new 是 String 类型的一个 关联函数

        io::stdin()  // stdin 函数返回一个 std::io::Stdin 的实例
            .read_line(&mut guess) // & (reference)
            .expect("Failed to read line"); // Result (成员：Ok, Err)

        // String -> u32
        // Shadowing
        // String实例的trim方法会去除字符串开头和结尾的空白字符串以及回车符等
        // String的parse方法将字符串转换为其他类型
        // 若无法转换返回 Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // 一个 match 由分支(arms)构成
            Ordering::Less => println!("Too small!"), // arm 包含 模式(pattern) 和表达式开头的值与分支模式匹配应该执行的代码
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }    
    }   
}

