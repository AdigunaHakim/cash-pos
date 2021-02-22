fn main() {
    //mutable
    let mut x = 5_000;
    println!("The value of x is: {}", x);
    x = 6_000;
    println!("The value of x is: {}", x);

    //shadowing
    let x = 5;

    println!("The value of x is: {}", x);

    let x = x + 1;

    println!("The value of x is: {}", x);

    let x = x * 2;

    println!("The value of x is: {}", x);

    //shadowing different data type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of space is: {}", spaces);
}
