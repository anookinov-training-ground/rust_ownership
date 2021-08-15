fn main() {
    variable_scope();
    string_scope();
    
    // let x = 5;
    // let y = x;
    // println!("The value of y is: {}", y);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("The value of s2 is: {}", s2);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    println!("s1 = {}, s3 = {}", s1, s3);

    let s4 = String::from("hello");
    let (s44, len4) = calculate_length(s4);
    println!("The length of '{}' is {}.", s44, len4);

    let s5 = String::from("hello");
    let len5 = calculate_length_ref(&s5);
    println!("The length of '{}' is {}.", s5, len5);

    let mut s6 = String::from("hello");
    change(&mut s6);

    let mut s7 = String::from("hello");
    {
        let r1 = &mut s7;
        println!("{}", r1);
    }
    let r2 = &mut s7;
    println!("{}", r2);

    let mut s8 = String::from("hello");
    let r3 = &s8; // no problem
    let r4 = &s8; // no problem
    println!("{} and {}", r3, r4);
    // r3 and r4 are no longer used after this point
    let r5 = &mut s8; // no problem
    println!("{}", r5);
    let r6 = &mut s8; // compile OK but no problem?
    println!("{}", r6);

    // let reference_to_nothing = dangle();
    let reference_to_thing = no_dangle();
    println!("{}", reference_to_thing);

    let mut s9 = String::from("hello world");
    let word = first_word(&s9); // word will get the value 5
    s9.clear(); // this empties the String, making it equal to ""
    println!("{}", word); //word still has the value 5 here, but there's no more string that we could meaningfully use the value 5 with. word is now totally invalid!

    let s10 = String::from("hello world");
    let hello = &s10[0..5];
    let world = &s10[6..11];
    let _slice1 = &s10[0..2];
    let slice1 = &s10[..2];
    let len10 = s10.len();
    let _slice2 = &s10[3..len10];
    let slice2 = &s10[3..];
    let _slice3 = &s10[0..len10];
    let slice3 = &s10[..];
    println!("hello = {}, world = {}", hello, world);
    println!("slice1 = {}", slice1);
    println!("slice2 = {}", slice2);
    println!("slice3 = {}", slice3);

    let s11 = String::from("hello world");
    let word_str = first_word_str(&s11);
    println!("{}", word_str);

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word2 = first_word_str2(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word3 = first_word_str2(&my_string_literal[..]);

    //because string literals *are* string slices already, this works too, without the slice syntax!
    let word4 = first_word_str2(my_string_literal);

    println!("word2 = {}, word3 = {}, and word4 = {}", word2, word3, word4);

    let a = [1, 2, 3, 4, 5];

    let slice_a = &a[1..3];

    assert_eq!(slice_a, &[2, 3]);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.
// Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped.

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

fn first_word_str2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_str(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length_ref(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the callling function
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. The backing memory if free.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn variable_scope() { // s is not valid here, it's not yet declared
    let s = "hello"; // s is valid from this point forward
    println!("The value of s is: {}", s);
} // this scope is now over, and s is no longer valid

fn string_scope() {
    let mut s = String::from("hello"); // s is valid from this point forward

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
} // this scope is now over, and s is no longer valid