// #! [allow(unused)] //lint
pub fn add(a: i32, b: i32) -> i32 {
    a + b

}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 3);
        assert_eq!(add(-5, -5), -10);
    }
    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
}
}
