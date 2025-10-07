struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // 这里没有分号


// 元组结构体： 没有具体的字段名，只有字段的类型

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 没有任何字段的类单元结构体

struct AlwaysEqual;

/* example */
// 派生 Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/* method */
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool { // 另外可以设计为 getter
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }
    /* 所有在 impl 块中定义的函数被称为 关联函数 */
    /* 可以定义不以 self 为第一参数的关联函数(因此不是方法) */

    fn square(size: u32) -> Self { // Self 首字母必须大写
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User{
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // 可以使用 . 访问某一个成员变量
    user1.email = String::from("anotheremail@example.com"); // 注意，此时user1 必须是mut

    // 结构体更新语法
    //let user2 = User {
    //    active: user1.active,
    //    username: user1.username,
    //    email: String::from("another@example.com"),
    //    sign_in_count: user1.sign_in_count,
    //};

    // 与下面写法等价
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // 这里没有逗号. 剩余未显示字段与给定实例相同
    };

    // 在创建 user2 后就不能再使用 user1 了，
    // 因为 user1 的 username 字段中的 String 被移到 user2 中 (move)
    // 若只复用user1的 active 和 sign_in_count， 则 user1 还可以继续使用
    //println!("user1 username: {}", user1.username); //无法运行, 被借用
    //println!("user1 email: {}",user1.email); // 可以运行，因为user1.email没有被借用
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;


    /* example */
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);

    dbg!(&rect1);

    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}.", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 5,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can tect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("Square is : {:#?}",sq);
}

// 字段初始化简写
fn build_user(email: String, username: String) -> User { // 参数与结构体中field名称相同
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}