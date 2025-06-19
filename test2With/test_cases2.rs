// tests/integration_test.rs

use my_project::{add, multiply}; // Import public functions from the lib

#[test]
fn test_add_function() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_multiply_function() {
    assert_eq!(multiply(4, 5), 20);
}
