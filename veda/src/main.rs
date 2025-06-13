#[allow(dead_code)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let int_point = Point { x: 1, y: 2 };
//     let float_point = Point { x: 1.1, y: 2.2 };

//     println!("Int: ({}, {})", int_point.x, int_point.y);
//     println!("Float: ({}, {})", float_point.x, float_point.y);
// }
enum Gender {
    Male,
    Female,
    Other,
}
enum Age{
    Child,
    Adult,
    Senior,
}
struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        gender: Gender::Female,
    };

    match person.gender {
        Gender::Male => println!("Male"),
        Gender::Female => println!("Female"),
        Gender::Other => println!("Other"),
    }
    match person.age {
        Age::Adult => println!("Adult"),
        Age::Child => println!("Child"),
        Age::Senior => println!("Senior"),
    }
}