// 控制流


fn main() {
    let number = 3;

    if number < 5 { // condition 必须是 bool, Rust 不会自动转换
        println!("condition was true!");
    } else {
        println!("condition was false!");
    }

    let condition = true;
    let number = if condition {5 * 40} else {6 * 79}; // 条件里是表达式，且都为i32
    println!("The value of number is: {number}");

    let mut counter = 0;

    let result = loop { // loop 无限循环
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    // 循环标签(loop label)， 在多个循环间消除歧义
    counter = 0;
    'counting_up: loop {
        println!("counter = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End counter = {counter}");

    // while 条件循环
    let mut n = 3;

    while n != 0 {
        println!("{n}!");
        n -= 1;
    }

    println!("LIFTOFF!!!");

    // for 遍历集合
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // 更安全的方式
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..=4).rev() { // (1..4) -> [1,3]; (1..=4) -> [1,4]
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
