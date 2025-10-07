fn main() {
    //println!("Hello, world!");

    /* Ownership */

    // String 类型管理被分配到堆上的数据
    // String 类型时可变的，而字符串字面值是不可变的
    let mut s = String::from("hello"); // 请求所需内存(memory allocator)

    s.push_str(", world!");

    println!("{s}");

    // 当变量离开作用域后，Rust 自动调用 drop 函数并清理变量的堆内存

    let s1 = s;  // named 'move' in Rust (shallow copy) -> double free

    println!("{s1}");

    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x; // 简单标量值存储在 stack 上, 且大小已知
    println!("x = {x}, y = {y}");


    let s = String::from("NUAA"); // s 进入作用域

    takes_ownership(s); // s 的值移动(move)到函数里
                        // 从这里开始不再有效'
    //println!("{s}");  // 这会报错

    let x = 5;

    make_copy(x);

    println!("{x}");

    let s1 = gives_ownership(); // some_string -> s1 (move)

    let s2 = String::from("hello"); // s2 enter scope

    let s3 = takes_and_gives_back(s2); // s2 -> function -> s3, s2 break down

    println!("s1 = {s1}, s3 = {s3}");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}");


    /* reference and borrow */

    // 引用(reference) 像一个指针, 是一个地址

    let s1 = String::from("hello, world!");

    let len = calculate_length_with_reference(&s1); // 允许使用值但不获取所有权
    // &String s 指向 String s1 

    println!("The length of '{s1}' is {len}.");

    // 将创建一个引用的行为称为 借用(borrowing), 只读不写(除非使用可变引用 &mut)

    let mut s = String::from("hello");

    change(&mut s); // 直接在 s 对应的堆上修改，无返回值

    println!("s = {s}");

    // 一个变量最多可同时存在一个可变引用
    // 在编译时避免数据竞争(data race)

    // 也不能在拥有不可变引用的同时拥有可变引用(保证读安全)
    // 但可以同时拥有多个不可变引用


    // 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("r1 = {r1}, r2 = {r2}"); // 这里使用后, r1, r2 便不再使用

    let r3 = &mut s;
    //println!("r1 = {r1}, r2 = {r2}, r3 = {r3}"); // 二者不能同时访存
    println!("r3 = {r3}");

    // 悬垂引用 (Dangling Referencs) : 释放内存时保留指向它的指针
    // let reference_to_nothing = dangle(); // 虽返回了s的引用,但对应的内存早已被释放

    // 直接返回 String
    let s = no_dangle();
    println!("return string is '{s}'.");



    /* Slice Type */
    /*
    切片（slice）允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一种引用，所以它不拥有所有权
     */

    /*
    编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串中找到的第一个单词。
    如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。
     */

    // [starting_index .. ending_index]
    // "字符串slice" 的类型声明写为 &str

    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5

    //s.clear();  // 这清空了字符串"", clear 中有 s 的可变引用(&mut)

    println!("the first word is: {word}."); // &mut 和 & 不可同时存在

    s.clear(); // 在 word scope 外是可行的 

    // let s = "Hello, world!"
    // s 的类型为 &str, 是一个指向二进制程序特定位置的slice。
    // 这解释了为什么字符串字面值是不可变的(immutable), &str 是一个不可变引用

    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`(的slice), 部分或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    // &my_string[..] 等价于 &my_string
    let word = first_word(&my_string);

    let my_string_literal = "hello world"; // 字面值

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 或者最简洁
    let word = first_word(my_string_literal);

    // 其他类型的 slice
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // slice type is &[i32]
    assert_eq!(slice, &[2, 3]);
}

// 所有权与函数
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}


// 返回值与作用域
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) { // 注意，这里没有使用返回值
    some_string.push_str(", world!");
}

//fn dangle() -> &String { // dangle() 返回一个字符串的引用
//    let s = String::from("hello"); // s 是一个新字符串

//    &s // 返回字符串 s 的引用
//} // 这里 s 离开作用域并被丢弃， 内存被释放
// Dangerous！

fn no_dangle() -> String {
    let s = String::from("NUAA again!");
    s
}

fn first_word(s: &str) -> &str { // "String slice" 的type 为 "&str"
    let bytes = s.as_bytes(); // as_bytes() 将 String 转化为字节数组

    // iter() 是数组上的迭代器，返回集合中的每个元素
    // enumerate() 将 iter 作为元组的一部分返回
    for (i, &item) in bytes.iter().enumerate() { // 指定模式, i 是索引, &item是单个字节
        if item == b' ' { // 通过字节的字面值语法来寻找代表空格的字节
            return &s[..i];
        }
    }

    &s[..]
}