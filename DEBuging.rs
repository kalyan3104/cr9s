// Print Debugging

fn main() {
    let x = 10;
    println!("x is: {}", x);
    dbg!(x); // adds file + line number
}
//////////////////////////////////////////////////////////////
// 
// thread 'main' panicked at 'oh no!', src/main.rs:2:5
// stack backtrace:
//    0: std::panicking::begin_panic
//    ...RUST_BACKTRACE=1 cargo run

// Backtrace Debugging
// fn main() {
//     let x = 10;
//     let y = 20;
//     let z = x + y;
//     dbg!(z); // adds file + line number
    // If this were a panic, it would show the backtrace
    // with file and line numbers for each function call
    // leading to the panic.
// }    

//////////////////////////////////////////////////////////////
/// Logging Debugging
/// add in cargo.toml
///  log = "0.4"
// env_logger = "0.10"
// use log::{info, warn};

// fn main() {
//     env_logger::init();
//     info!("This is info");
//     warn!("This is a warning");
// }

////////////////////////////////////////////////////////////
/// tool clippy
/// cargo clippy (cmd to run clippy)
/// clippy is a linter for Rust code that helps catch common mistakes and improve code quality
/// It provides suggestions for better practices and can help you write more idiomatic Rust code.
/// To use clippy, you need to add it as a dependency in your Cargo.toml
/// and then run `cargo clippy` to check your code for issues.
/// Clippy will analyze your code and provide warnings and suggestions for improvement.
/// You can then fix the issues and run clippy again to ensure your code is clean and follows best practices.
////// Clippy is a powerful tool that can help you write better Rust code and catch potential bugs early
/// in the development process. It is highly recommended to use clippy as part of your Rust
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Sanitizer-Based Debugging
/// Catch issues like memory leaks, data races (nightly only).
/// [profile.dev]
// sanitize = ["address"]

// cargo run -Zsanitizer=address
///////////////////////////////////////////////////////////////////////////
/// Fuzzing (cargo-fuzz)
// Debug unexpected crashes or behavior under random input.
/////////////////////////////////////////////////////////////////////
// Temporary debugging
/// #![allow(unused)]
/// #![] = "Inner Attribute" (applies to the entire crate/module)

// allow() = "Lint Attribute" (controls compiler warnings)

// unused = "Lint Name" (specific warning to suppress) 
// #[allow(lint_name)]	Suppresses a lint warning
// #[warn(lint_name)]	Elevates a lint to warning level
// #[deny(lint_name)]	Turns a lint into a compile-time error
// #[forbid(lint_name)]	Same as deny, but cannot be overridden later

// and ther are two types of syntax 
// Syntax                    	Scope
// #![allow(...)]	Applies to entire crate/file (inner)
// #[allow(...)]	Applies to next item (outer)

// Feature - #![allow(...)] (Inner)	    #[allow(...)] (Outer)
// Scope -	Entire file/crate              	Next item only
// Placement - Top of file (with #!)	  Above the item it modifies
// Overrideable? -	No (file-wide)   	Yes (can be overridden locally)

// When to Use Each
// #![allow(...)]

// When you want to silence a warning globally (e.g., for prototyping).

// Example: #![allow(dead_code)] in a draft module.

// #[allow(...)]

// When you want to allow a warning for one specific item.

// Example: #[allow(clippy::too_many_arguments)] on a complex function.
//Lint Names
// A. Default Lints (Standard Library)
// Correctness
// arithmetic_overflow

// unconditional_recursion

// unreachable_code

// unsafe_code

// unused_unsafe

// Style
// non_snake_case

// non_upper_case_globals

// unused_variables

// unused_mut

// unused_imports

// Deprecation
// deprecated

// unstable_features

// Experimental
// incomplete_features

// B. Clippy Lints (Partial List)
// Performance
// clippy::needless_range_loop

// clippy::redundant_clone

// clippy::unnecessary_lazy_evaluations

// Style
// clippy::too_many_arguments

// clippy::neg_multiply

// clippy::manual_memcpy

// Complexity
// clippy::cyclomatic_complexity

// clippy::too_many_lines

//3. Special Lint Groups
// Group	Effect
// #[warn(warnings)]	Warn for all lints
// #[deny(warnings)]	Error for all lints
// #[allow(warnings)]	Suppress all warnings
// #[warn(rust_2018_idioms)]	Edition-specific best practices

// 4. Tool-Specific Lints
// Rustdoc
// rustdoc::broken_intra_doc_links

// rustdoc::missing_crate_level_docs

// Compiler Plugins
// unsupported_lint_name (for custom lints)
// example:
// #![allow(unused_variables)] // File-wide
// #![warn(missing_docs)]      // Warn if no docs

// #[deny(unsafe_code)]        // Error on `unsafe`
// mod safe_module {
//     /// This function is documented!
//     pub fn foo() {
//         let x = 5;          // No warning (allowed globally)
//     }
// }

// #[allow(clippy::needless_range_loop)]
// fn bar() {
//     for i in 0..10 { /*...*/ } // Clippy lint silenced
// }
