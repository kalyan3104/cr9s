macro_rules! crab_say {
    ($msg:expr) => {
        println!("🦀 says: {}", $msg);
    };
}

fn main() {
    crab_say!("I love macros!"); // Expands to println!("🦀 says: I love macros!");
}

///////////////////////////////////////////////////////////

// fn main() {
//     // Without macro (verbose approach)
//     let mut v1 = Vec::new();
//     v1.push(1);
//     v1.push(2);
//     v1.push(3);
//     println!("{:?}", v1);  // Prints: Manually created vector: [1, 2, 3]

//     // With macro (concise approach)
//     let v2 = vec![1, 2, 3];
//     println!("{:?}", v2);     // Prints: Macro-created vector: [1, 2, 3]

//     // Bonus: They're identical in functionality!
//     assert_eq!(v1, v2);
//     println!("Both vectors are equal! �");
// }
