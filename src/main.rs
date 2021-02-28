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
    let another_text = &some_text;
    println!("The value of another_text is : {}", another_text);
    println!("The value of some_text again is : {}", some_text);
}