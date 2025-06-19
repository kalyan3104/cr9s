// buggy.rs
fn main() {
    let numbers = vec![10, 20, 30];  // Line 2
    let idx = 3;             
    dbg!(&numbers);  // Prints to stderr even in gdb        // Line 3 
    println!("Value: {}", numbers[idx]); // Line 4 (panic here)
}
//////////////////
// rust-gdb ./buggy
// (gdb) break buggy.rs:3  # Stop at `let idx = 3`
// (gdb) run
// (gdb) next  # Advance to line 4 (but don't execute yet)
// (gdb) print numbers
// $1 = vec![10, 20, 30]  # Success!
// (gdb) next  # Now trigger the panic
