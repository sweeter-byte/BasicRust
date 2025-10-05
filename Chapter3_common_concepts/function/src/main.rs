// function

// 在Rust中，函数和变量名使用 snake case 规范风格，即所有字母都是小写并使用下划线分割单词

// 函数由一系列语句和一个可选的结尾表达式组成
// Statements 执行一些操作但不返回值得指令
// Expressions 计算并产生一个值
fn main() {
    println!("Hello, world!");

    another_function();
    print_value(7);
    print_labeled_measurement(5, 'h');

    let y = {
        let x =3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn print_value(x: i32) {
    println!("The value of input is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 具有返回值的函数

fn five() -> i32 {
    5
}
