enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Red;

    match light {
        TrafficLight::Red => println!("Stop 🛑"),
        TrafficLight::Yellow => println!("Caution ⚠️"),
        TrafficLight::Green => println!("Go 🟢"),
    }
}
//////////////////////////////////////////////////////////
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn main() {
    let shape = Shape::Rectangle(10.0, 5.0);

    match shape {
        Shape::Circle(radius) => println!("Circle with radius {}", radius),
        Shape::Rectangle(width, height) => println!("Rectangle: {} x {}", width, height),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {}, {}", a, b, c),
    }
}
//////////////////////////////////////////////////////////////////////////////////////////
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("Move to coordinates: ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}
///////////////////////////////////////////////////////////////////////
use std::collections::HashMap;

enum ComplexEnum<T> {
    Value(T),
    List(Vec<T>),
    Map(HashMap<String, T>),
    Nested(Box<ComplexEnum<T>>),
}

fn main() {
    let example = ComplexEnum::Nested(Box::new(ComplexEnum::Value(42)));

    match example {
        ComplexEnum::Value(val) => println!("Single value: {}", val),
        ComplexEnum::List(vec) => println!("List: {:?}", vec),
        ComplexEnum::Map(map) => println!("Map with {} entries", map.len()),
        ComplexEnum::Nested(inner) => match *inner {
            ComplexEnum::Value(v) => println!("Nested value: {}", v),
            _ => println!("Nested something else"),
        },
    }
}

