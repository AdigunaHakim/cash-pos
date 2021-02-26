fn main() {
    //integer literals
    let mut x: u32 = 98_222;
    println!("value is integer of {}", x);
    x = 0xff;
    println!("value is integer of {}", x);
    x = 0o77;
    println!("value is integer of {}", x);
    x = 0b1111_0000;
    println!("value is integer of {}", x);
    x = b'A'.into();
    println!("value is integer of {}", x);

    //float
    let y = 2.0; // f64
    println!("value is float of {}", y);
    let y: f32 = 3.0; // f32
    println!("value is float of {}", y);

    //processing number
    let sum = 5 + 10;
    println!("value of addition 5 + 10 = {}", sum);
    let difference = 95.5 - 4.3;
    println!("value of subtraction 95.5 - 4.3 = {}", difference);
    let product = 4 * 30;
    println!("value of multiplication 4 * 30 = {}", product);
    let quotient = 56.7 / 32.2;
    println!("value of division 56.7 / 32.2 = {}", quotient);
    let remainder = 43 % 5;
    println!("value of modulus 43 % 5 = {}", remainder);

    //char
    let c = 'z';
    println!("value is char of {}", c);
    let z = 'ℤ';
    println!("value is char of {}", z);
    let heart_eyed_cat = '😻';
    println!("value is char of {}", heart_eyed_cat);

    //tuple
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup; //destruction
    println!("The value of y is: {} same with {}", y, tup.1);

    //array
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("The first value of months is : {}", months[0]);
    let array1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The first value of array1 is : {}", array1[0]);
    let array2 = [3; 5];
    println!("The first value of array2 is : {}", array2[0]);
}
