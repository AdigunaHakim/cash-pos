fn main() {
    //this is literal string save on stack
    let literal_string = "hello";
    println!("The value of literal_string is : {}", literal_string);

    //data type String different with string literal, save on stack & heap
    //heap contains of three parts. pointer, length & capacity
    let mut some_text = String::from("hello");
    some_text.push_str(", world!");
    println!("The value of some_text is : {}", some_text);

    //assign some_text to another_text with ampersand means borrows reference value 
    //when assign without ampersand means value of some_text no longer exists move to another_text
    //when assign without ampersand some_text was invaidated its to prevent double free error
    let another_text = &some_text;
    println!("The value of another_text is : {}", another_text);
    println!("The value of some_text again is : {}", some_text);

    clone_string();

    //when some_text pass as param on func takes_ownership, some_text is no longer valid and move value to function
    takes_ownership(some_text);
    //println!("The value of some_text again is : {}", some_text);

    let num = 5;
    makes_copy(5);
    //when num pass param on func makes_copy, value num still valid because num is integer and save on stack
    println!("The value of num is : {}", num);

    let some_string = gives_ownership();
    let some_string_again = gives_ownership();
    println!("{} {}", some_string, some_string_again);

    let another_string = takes_and_gives_back(some_string);
    //when some_string pass param on func takes_and_gives_back, is no longer valid will give ownership
    //println!("{} {}", some_string, another_string);
    println!("{}", another_string);

    //pass s1 to param of func calculate_length and give ownership value, and return to tuple
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn clone_string() {
    let s1 = String::from("hello");

    //deep copy use clone 
    //deep & shallow copy ???
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) { 
    println!("{}", some_integer);
} 


fn gives_ownership() -> String {    
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String { 
    a_string 
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}