fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const N: u32 = 5;
    const STATIC: &'static str = "string static constant";
    const CONST: &str = "string constant";
    let immutable = "string immutable";

    println!("{}",N);
    println!("{}",STATIC);
    println!("{}",CONST);
    println!("{}",immutable);

    //this part prints 12 since it takes the previous x, uses it, then stores it as a new x
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces:usize = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
