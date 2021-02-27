fn main() {
    let x = 6;
    let y = {
        let x = 3;
        x
    };

    another_function(x, y);
    println!("The value of function five() is {}", five());
    println!("The value of function plus_one() is {}", plus_one(five()));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32  {
    x + 1
}
