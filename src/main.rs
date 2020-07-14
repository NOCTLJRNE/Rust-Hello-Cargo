fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    };
    let user1 = User {
        username: String::from("abc_user_id"),
        email: String::from("abc@gmail.com"),
        active: true,
        sign_in_count: 1,
    };
    let mut s = String::from("hello July 14th");
    let a = [10, 20, 30, 40, 50];
    let a_sliced = &a[1..3];
    let number: i32 = 5;
    let r1 = &mut s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    let r3 = &mut s;
    println!("r3: {}", r3);
    let r4 = &mut s;
    println!("r4: {}", r4);
    println!("calling s from top scope: {}", s);
    println!("another call s from top scope: {}", s);
    let len = calculate_length_ref(&s);
    println!("REF, length of '{}' is: {}", s, len);
    change(&mut s);
    // let len = calculate_length_move(s); // this would move s out ofscope
    // println!("MOVE, length of s is: {}", len);
    let word = first_word(&s);
    println!("first word is: {}", word);
    takes_ownership(s);
    println!("Here's my number: {}", number);
    makes_copy(number);
    println!("My number is still there: {}", number);
    // println!("calling s from top scope: {}", s); // this won't work , Copy trait not implemented
    println!("Hello, world v2!");
    for (i, element) in a.iter().enumerate() {
        println!("the value of {} is: {}", i, element);
    }
}
fn takes_ownership(some_string: String) {
    println!("no longer yours: {}", some_string);
}
fn makes_copy(some_interger: i32) {
    println!("I'm still yours: {}", some_interger)
}
fn calculate_length_ref(string_ref: &String) -> usize {
    string_ref.len()
}
fn calculate_length_move(string_ref: String) -> usize {
    string_ref.len()
}
fn change(string_ref: &mut String) {
    string_ref.push_str(", world");
}
fn dangle() -> String {
    let s = String::from("The void");
    s
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        println!("letter: {}", item);
        if item == b' ' {
            return &s[0..i];
        }
        // return &s[..];
    }
    &s[..]
}
