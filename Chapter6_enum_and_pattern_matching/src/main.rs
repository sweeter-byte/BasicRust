#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr_v1 {
    V4(String),
    V6(String),
}

// 每个变体可以处理不同类型和数量的数据
#[derive(Debug)] 
enum IpAddr_v2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,  // 没有关联任何数据
    Move {x: i32, y: i32},  // 类似结构体包含命名字段
    Write(String), // 包含String
    ChangeColor(i32, i32, i32), // 包含三个 i32
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn describle_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for Americal!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(v) => Some(v + 1),
        None => None,
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // IpAddrKind::V4和IpAddrKind::V6都是IpAddrKind类型
    // 可以定义一个函数处理接受这两种不同的值

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:#?}", home);
    println!("{:#?}", loopback);

    let home_v1 = IpAddr_v1::V4(String::from("127.0.0.1"));
    let loopback_v1 = IpAddr_v1::V6(String::from("::1"));

    // 构造函数会自动被定义

    println!("{:#?}", home_v1);
    println!("{:#?}",loopback_v1);

    // 每个变体所处理的数据类型和数量可以不一样
    // 数据类型是任意的，甚至可以是另外的结构体和枚举
    println!("another way:");

    let home_v2 = IpAddr_v2::V4(127, 0, 0, 1);
    let loopback_v2 = IpAddr_v2::V6(String::from("::1"));

    println!("{:#?}", home_v2);
    println!("{:#?}", loopback_v2);

    // 使用 Option 枚举
    let some_number = Some(5); // Option<i32>
    let some_char = Some('e'); // Option<char>

    let absebt_number: Option<i32> = None; // 对于 None 必须显式指明类型

    // 注意，Option<T> 和 T 是不同类型的值， 二者不能直接做运算
    // 因为 T 是一个确切的类型， 对应的变量一定是有值的
    // 但 Option<T> 则不然， 对应的变量有可能是空值， 在进行运算前需先转换为 T 

    // 可以使用 模式匹配 安全取值
    let x: Option<i32> = Some(10);

    match x {
        Some(v) => println!("the value of x is {}", v),
        None => println!("None!"),
    }

    // 或者是

    if let Some(v) = x {
        println!("the value of x is {}", v);
    } else {
        println!("None");
    }

    let my_coin: Coin = Coin::Penny;
    let value: u8 = value_in_cents(my_coin);
    println!("the value of my coin is {}.", value);

    let another_coin: Coin = Coin::Quarter(UsState::Alabama);
    let another_value: u8 = value_in_cents(another_coin);
    println!("the value of another coin is {another_value}.");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(v) = six {
        println!("the value of six is {v}.");
    } else {
        println!("None");
    }

    if let Some(v) = none {
        println!("the value of none is {v}.");
    } else {
        println!("None");
    }

    // 通配符other 与 _ 占位符
    // 在 match 中必须放在最后

    // match 与 if let 
    // if let 是 match 的一个语法糖
    let coin: Coin = Coin::Quarter(UsState::Alabama);
    let mut first_count = 0;
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => first_count += 1,
    }
    println!("firsr count is {first_count}.");

    let mut second_count = 0;
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {state:?}!");
    } else {
        second_count += 1;
    }
    println!("second count is {second_count}.");


}
