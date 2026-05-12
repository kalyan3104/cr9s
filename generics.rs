// function
fn print_value<T: std::fmt::Debug>(val: T) {
    println!("{:?}", val);
}

fn main() {
    print_value(42);
    print_value("Hello");
} 
/////////////////////////////////////////////////
//struct
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let int_point = Point { x: 1, y: 2 };
    let float_point = Point { x: 1.1, y: 2.2 };

    println!("Int: ({}, {})", int_point.x, int_point.y);
    println!("Float: ({}, {})", float_point.x, float_point.y);
}
//////////////////////////////////////////////////////////////////////////////
// enum

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let x: Option<i32> = Option::Some(10);
    let y: Option<&str> = Option::None;

    match x {
        Option::Some(val) => println!("Some value: {}", val),
        Option::None => println!("No value"),
    }
}
/////////////////////////////////////////////////////////////////////////////////////////////
struct Pair<T, U> {
    first: T,
    second: U,
}

fn main() {
    let pair = Pair { first: "Rust", second: 2025 };
    println!("Pair: ({}, {})", pair.first, pair.second);
}
/////////////////////////////////////////////////////////////////////////////
//trait

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }ś
    max
}

fn main() {
    let nums = vec![10, 20, 5, 30];
    println!("Largest: {}", largest(&nums));
}
//////////////////////////////////////////////////////////////////////////////////////////
//trait impl
trait Summarize {
    fn summary(&self) -> String;
}

struct Article<T> {
    title: T,
}

impl<T: std::fmt::Display> Summarize for Article<T> {
    fn summary(&self) -> String {
        format!("Article: {}", self.title)
    }
}

fn main() {
    let post = Article { title: "Generics in Rust" };
    println!("{}", post.summary());
}
/////////////////////////////////////////////////////////////////////////////////////////////////
// lifetimes+generics
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let a = "Rustacean";
    let b = "🦀";
    println!("Longest: {}", longest(a, b));
}
