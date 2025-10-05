fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    // constant
    // 不能使用 mut, 使用 const 而不是 let 进行声明
    // 必须注明值的类型
    // 只能是常量表达式 (hard code)
    // 一般常量名大写

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing

    let y = 89;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    } // end scope

    println!("The value of y is: {y}");
}
