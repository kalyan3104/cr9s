fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // Ownership moves to s2

    // println!("{}", s1); // ❌ Error: s1 is no longer valid
    println!("{}", s2); // ✅ Works
}
//////////////////////////
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1.clone(); // Deep copy

    println!("s1 = {}, s2 = {}", s1, s2); // ✅ Both work
}
//////////////////////////////////////////////////////////
fn takes_ownership(s: String) {
    println!("Got string: {}", s);
}

fn main() {
    let s = String::from("Ownership");
    takes_ownership(s); // s is moved here

    // println!("{}", s); // ❌ Error: s is moved
}
///////////////////////////////////////////////////////////
fn gives_ownership() -> String {
    String::from("Returned")
}

fn main() {
    let s = gives_ownership(); // Now s owns the String
    println!("{}", s); // ✅ Works
}
////////////////////////////////////////////////
fn calculate_length(s: &String) -> usize {
    s.len() // Read-only
}

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Pass a reference

    println!("The length of '{}' is {}.", s1, len); // ✅ s1 still valid
}
///////////////////////////////////////////////////////
fn change(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let mut s = String::from("hello");
    change(&mut s); // Mutable reference

    println!("{}", s); // ✅ Output: hello world
}
////////////////////////////////////////////////
fn main() {
    let mut s = String::from("Hi");

    let r1 = &mut s;
    // let r2 = &mut s; // ❌ Only one mutable reference allowed

    println!("{}", r1);
}
///////////////////////////////////////////////////////////
fn main() {
    let s = String::from("Rust");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2); // ✅ Allowed
}
///////////////////////////////////////////////////////////////
fn main() {
    let mut s = String::from("Rust");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // ❌ Cannot borrow as mutable because immutable references exist

    println!("{}, {}", r1, r2);
}
////////////////////////////////////////////////////////
struct Book {
    title: String,
}

fn main() {
    let b1 = Book {
        title: String::from("Rust Book"),
    };

    let b2 = b1; // b1 is moved

    // println!("{}", b1.title); // ❌ Error
    println!("{}", b2.title); // ✅
}
//////////////////////////////////////////////////////////////////////-+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++6
fn main() {
    let x = 5;
    let y = x; // Copy trait applies (i32)

    println!("x = {}, y = {}", x, y); // ✅ Both valid
}
//////////////////////////////////////////////////////////////
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let x = String::from("long");
    let y = "short";
    let result = longest(&x, y);
    println!("Longest: {}", result);
}
/////////////////////////////////////////////////////////////////////////
