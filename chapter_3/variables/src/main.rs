fn main() {
    // 定数
    const MAX_POINTS: u32 = 100_000;

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of first of tup is: {}", tup.0); // 添字アクセス

    // array
    let arr = [1,2,3,4,5];
    let first = arr[0];
    let second = arr[1];
    //let index = 10;
    //let element = arr[index];
    //println!("The value of element is: {}", element);   // panic!

}
