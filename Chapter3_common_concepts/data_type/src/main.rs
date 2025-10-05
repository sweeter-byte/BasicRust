// data_type
// there are two subtypes

// scalar and compound

/*
scalar

整型：{i, u} * {8, 16, 32, 64, 128, size}

浮点型：f32, f64

布尔：bool (true, false)

字符类型：char 声明时使用单引号(''), 大小为四个字节，表示Unicode


*/


/*
compound types
元组(tuple) : 一旦声明，长度不能改变。每个元素类型可以不同

数组(array) : 数组中每个元素的类型必须相同，长度固定




*/
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    println!("sum: {sum} \ndifference: {difference}\nproduct: {product}\nquotient: {quotient}\ntruncated: {truncated}\nremainder: {remainder}");


    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 解构
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("The first one is: {}", tup.0);


    let a = [1, 2, 3, 4, 5];
    let b: [i32;5] = [1, 2, 3, 4, 5];
    let c = [3; 5];

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);

    let first = a[0];
    let second = a[1];

    //let ten = a[9];
    // panic: 程序因为错误而退出，只有在运行时才能发下

    println!("in a, first is {first}, second is {second}");

}
