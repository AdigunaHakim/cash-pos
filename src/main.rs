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

    println!("The value of func statement_if is: {}", do_if());
    do_loop();
    println!("The value of func do_loop_with_value is: {}", do_loop_with_value());
    do_while();
}

//function with return value
fn do_if() -> i32 {
    let condition = true;

    //assign with if expresion
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
    number
}

//function loop
fn do_loop(){
    let mut x = 0;
    loop {
        println!("Do loop again! {}", x);
        x += 1;
        if x > 10 {
            break;
        }
    }
}

//function loop with return value
fn do_loop_with_value() -> i32{
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {}", result);
    result
}

//do while load value of arrays
fn do_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value of arrays index[{}] is: {}", index, a[index]);

        index += 1;
    }
}
