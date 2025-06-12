// 1
struct Person {
    name: String,
    age: u8,
}

// 2
struct Point {
    x: i32,
    y: i32,
}

// 3
struct Book {
    title: String,
    pages: u32,
}

// 4
struct Rectangle {
    width: u32,
    height: u32,
}

// 5
struct Student {
    id: u32,
    name: String,
    grade: char,
}

// 6
struct Temperature {
    celsius: f64,
}

// 7
struct Car {
    brand: String,
    year: u16,
}

// 8
struct Circle {
    radius: f64,
}

// 9
struct Movie {
    title: String,
    rating: f32,
}

// 10
struct Coordinates {
    latitude: f64,
    longitude: f64,
}
////////////////////////////////////////////////////////////////////////////////////////////////
// 11
struct Address {
    street: String,
    city: String,
    zip: u32,
}

// 12
struct User {
    username: String,
    email: String,
    active: bool,
}

// 13
struct Laptop {
    brand: String,
    specs: Specs,
}

struct Specs {
    ram: u32,
    storage: u32,
}

// 14
struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

// 15
struct Employee {
    name: String,
    id: u32,
    department: Option<String>,
}

// 16
struct Account<'a> {
    holder: &'a str,
    balance: f64,
}

// 17
struct Company {
    name: String,
    employees: Vec<Employee>,
}

// 18
struct Config {
    debug: bool,
    max_connections: u32,
    timeout: u64,
}

// 19
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

// 20
struct Pair<T> {
    first: T,
    second: T,
}
////////////////////////////////////////////////////////////////////////////
// 21 to 50 — All follow a similar pattern with generics and traits
use std::collections::HashMap;

struct ComplexType<T: std::fmt::Debug + Clone> {
    data: Vec<T>,
    metadata: HashMap<String, T>,
}

impl<T: std::fmt::Debug + Clone> ComplexType<T> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}












