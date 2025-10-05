use std::io;

fn main() {
    
    //change_temperature();
    //fab_test();
    //song();'
    table();

}

fn table() {
    println!("Hello, there are answers of Chapter3!");
    println!("There 3 questions, please enter the number of question!");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line!");

    let choice: u8 = match choice.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a number!");
            return;
        }
    };

    if choice == 1 {
        change_temperature();
    } else if choice == 2 {
        fab_test();
    } else if choice == 3 {
        song();
    } else {
        println!("Please enter the number between 1 and 3!");
    }
}

fn change_temperature() {
    println!("What kind of temperature do you input?(C/F):");

    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line!");

    let temperature = temperature.trim().to_uppercase();

    if temperature == "C" {
        println!("You choose Celsius!");
        println!("Please input the number:");

        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line!");


        let num: f64 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter number!");
                return;
            }
        };

        println!("You enter the number: {num}");

        println!("The Fahrenheit is: {:.2}", num * 1.8 + 32.0);
    } else if temperature == "F" {
        println!("You choose Fahrenheit!");
        println!("Please input the number:");

        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line!");

        let num: f64 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter number!");
                return;
            }
        };

        println!("You enter the number: {num}");

        println!("The Celsius is: {:.2}", (num - 32.0) * 5.0 / 9.0);
    } else {
        println!("Please input 'C' or 'F'!!!");
    }
}

fn fab(n: i64) -> i64 {
    if n <= 0 {
        0
    }else if n == 1 || n == 2 {
        1
    } else {
        fab(n - 1) + fab(n - 2)
    }
}

fn fab_optimized(n: i64) -> i64 {
    if n <= 0 {
        0
    } else if n == 1 || n == 2{
        1
    } else {
        let mut a = 1;
        let mut b = 1;
        for _ in 3..=n {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }
}

fn fab_test() {
    let mut num = String::new();
    println!("Please input the value of n: \n I advise you enter the number smaller than 50!");
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line!");

    let n: i64 = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a number !");
            return;
        }
    };

    let result: i64 = fab_tail(n,0,1);
    println!("You input {n}, and the answer is {result}");
}

fn fab_tail(n: i64, a: i64, b: i64) -> i64 {
    if n == 0 {
        a
    } else {
        fab_tail(n - 1, b, a + b)
    }
}

fn song() {
    // 第 n 天对应的序数词（小写，与常见歌词一致）
    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    // 每日收到的礼物（注意顺序是从第 1 天到第 12 天）
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for day in 0..12 {
        // 歌曲开头两行（每一节相同模式）
        println!("On the {} day of Christmas", ordinals[day]);
        println!("my true love sent to me:");

        // 从当日向回列出礼物（倒序），并处理第 1 天前面不加 "and"
        for i in (0..=day).rev() {
            // 当 i == 0 且 day > 0 时，需要在 "a Partridge..." 前加 "and"
            if i == 0 && day > 0 {
                println!("and {}", gifts[0]);
            } else {
                println!("{}", gifts[i]);
            }
        }

        // 每节之间空一行
        println!();
    }
}
