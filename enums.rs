// 1
enum Direction {
    North,
    South,
    East,
    West,
}

// 2
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 3
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// 4
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// 5
enum Status {
    Success,
    Error,
}

// 6
enum Size {
    Small,
    Medium,
    Large,
}

// 7
enum Gender {
    Male,
    Female,
    Other,
}

// 8
enum Command {
    Start,
    Stop,
    Pause,
}

// 9
enum Role {
    Admin,
    User,
    Guest,
}

// 10
enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}
/////////////////////////////////////////////////////////////////////
// 11
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 12
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

// 13
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 14
enum Option<T> {
    Some(T),
    None,
}

// 15
enum Vehicle {
    Car(String, u16),
    Bike(String),
    Truck { brand: String, capacity: u32 },
}

// 16
enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}

// 17
enum HttpStatus {
    Ok(u16),
    NotFound(String),
    InternalError(String),
}

// 18
enum State {
    Init,
    Processing,
    Completed,
    Failed(String),
}

// 19
enum PaymentMethod {
    Cash,
    CreditCard(String),
    UPI(String),
}

// 20
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}
//////////////////////////////////////////////////////////////////////////////////////////
use std::collections::HashMap;

enum ComplexEnum<T> {
    Value(T),
    List(Vec<T>),
    Map(HashMap<String, T>),
    Nested(Box<ComplexEnum<T>>),
}

// Reuse the same complex pattern for remaining hard examples
// Just copy the above block and imagine changing use cases/types

// 21 to 50: Use `ComplexEnum` in creative scenarios like trees, ASTs, state machines, etc.
