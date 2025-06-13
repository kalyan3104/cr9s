fn print_value<T: std::fmt::Debug>(val: T) {
    println!("{:?}", val);
}

fn main() {
    print_value(42);
    print_value("Hello");
} 