fn main() {
    let number = 11;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    println!("The value of func statement_if is: {}", statement_if());
}

//function  with return value
fn statement_if() -> i32 {
    let condition = true;

    //assign with if expresion
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
    number
}
