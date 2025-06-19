// // // Option ENUM
// fn main() {
//     let _maybe_name: Option<String> = Some(String::from("Alice"));
//     let _no_name: Option<String> = None;
//     println!("Maybe name: {:?}", _maybe_name);
//     // println!("No name: {:?}", _no_name);
// }
// fn find_egg(has_egg: bool) -> Option<&'static str> {
//     if has_egg {
//         Some("🥚 Egg found!")  // Chef is happy!
//     } else {
//         None  // Chef panics: "No eggs?!"
//     }
// }

// fn main() {
//     let egg_status = find_egg(true);
//     match egg_status {
//         Some(egg) => println!("Let's bake! {}", egg),
//         None => println!("Run to the store!"),
//     }
// }
///////////////////////////////////////////////////////////////////////////////////////////
// fn get_ingredient(available: bool) -> Option<&'static str> {
//     if available {
//         Some("🍎 Apple")  // Got an apple!
//     } else {
//         None  // No apple :(
//     }
// }

// fn main() {
//     // Case 1: Ingredient available (Some)
//     let apple = get_ingredient(true);
//     match apple {
//         Some(fruit) => println!("Found: {}", fruit),  // "Found: 🍎 Apple"
//         None => println!("No fruit in stock!"),
//     }

//     // Case 2: Ingredient unavailable (None)
//     let missing_apple = get_ingredient(false);
//     match missing_apple {
//         Some(fruit) => println!("Found: {}", fruit),
//         None => println!("No fruit in stock!"),  // "No fruit in stock!"
//     }
// }
//////////////////////////////////////////////////////////////////////////////////////////
// Definition
// Result is another built-in Rust enum with two possible variants:

// Ok(value) → "Success! Here’s your result."

// Err(error) → "Oops, something went wrong. Here’s the error."
// enum Result<T, E> {  // <T> = success type, <E> = error type
//     Ok(T),   // Contains the successful value
//     Err(E),  // Contains an error
// }
///////////////////////////////////////////////////////////////////////////////

// Result
// fn cook_dish(success: bool) -> Result<&'static str, &'static str> {
//     if success {
//         Ok("🍕 Delicious pizza!")  // Success case
//     } else {
//         Err("🔥 Burnt the food!")  // Error case
//     }
// }

// fn main() {
//     // Case 1: Cooking succeeds (Ok)
//     let meal = cook_dish(true);
//     match meal {
//         Ok(dish) => println!("Success: {}", dish),
//         Err(e) => println!("Error: {}", e),
//     }

//     // Case 2: Cooking fails (Err)
//     let failed_meal = cook_dish(false);
//     match failed_meal {
//         Ok(dish) => println!("Success: {}", dish),
//         Err(e) => println!("Error: {}", e),  // This runs
//     }
// }
////////////////////////////////////////////////////////////////////

// fn cook_dish(success: bool) -> Result<&'static str, &'static str> {
//     if success {
//         Ok("🍕 Pizza is ready!")
//     } else {
//         Err("🔥 Burnt the food!")
//     }
// }

// fn main() {
//     // Case 1: Success (unwrap extracts "🍕 Pizza is ready!")
//     let good_meal = cook_dish(true).unwrap();
//     println!("{}", good_meal);  // Prints "🍕 Pizza is ready!"

//     // Case 2: Failure (unwrap PANICS!)
//     let bad_meal = cook_dish(false).unwrap();  // 💥 Thread crashes with "🔥 Burnt the food!"
// }
///////////////////////////////////////////////////////////////////////////////////////////////////
// fn cook_dish(success: bool) -> Result<&'static str, &'static str> {
//     if success {
//         Ok("🍕 Pizza is ready!")
//     } else {
//         Err("🔥 Burnt the food!")
//     }
// }

// fn main() {
//     // 1. Basic unwrap() - will panic on error
//     // let meal = cook_dish(false).unwrap(); // Uncomment to see panic
    
//     // 2. Safer alternatives:
    
//     // Option 1: Use match for proper error handling
//     println!("=== Using match ===");
//     match cook_dish(true) {
//         Ok(dish) => println!("Success: {}", dish),
//         Err(e) => println!("Error: {}", e),
//     }
    
//     // Option 2: unwrap_or with default value
//     println!("\n=== Using unwrap_or ===");
//     let meal1 = cook_dish(false).unwrap_or("🍞 Bread");
//     println!("Meal: {}", meal1);
    
//     // Option 3: unwrap_or_else with error handling
//     println!("\n=== Using unwrap_or_else ===");
//     let meal2 = cook_dish(false).unwrap_or_else(|err| {
//         println!("Logging error: {}", err);
//         "🥗 Salad"
//     });
//     println!("Meal: {}", meal2);
    
//     // Option 4: Using ? operator in a function
//     println!("\n=== Using ? operator ===");
//     fn serve_meal() -> Result<(), &'static str> {
//         let dish1 = cook_dish(true)?;
//         println!("First course: {}", dish1);
        
//         let dish2 = cook_dish(false)?; // This will return early
//         println!("Second course: {}", dish2); // Won't execute
        
//         Ok(())
//     }
    
//     match serve_meal() {
//         Ok(_) => println!("Full meal served successfully!"),
//         Err(e) => println!("Couldn't serve full meal: {}", e),
//     }
    
//     // Option 5: Convert to Option with ok()
//     println!("\n=== Converting to Option ===");
//     let maybe_dish = cook_dish(false).ok();
//     println!("Maybe dish: {:?}", maybe_dish);
    
//     // Only use unwrap() when absolutely certain
//     println!("\n=== Safe unwrap() usage ===");
//     let guaranteed_meal = cook_dish(true).unwrap(); // We know this will succeed
//     println!("Guaranteed meal: {}", guaranteed_meal);
// }
//////////////////////////////////////////////////////////////////

// // The Kitchen API
// fn make_pizza(has_ingredients: bool) -> Result<String, String> {
//     if has_ingredients {
//         Ok("🍕 Delicious Pizza!".to_string())
//     } else {
//         Err("🔥 Burnt Pizza Disaster!".to_string())
//     }
// }

// fn make_backup_meal() -> String {
//     "🍝 Pasta".to_string()
// }

// // 1. The Dangerous Chef (DON'T DO THIS)
// fn dangerous_chef() {
//     println!("\n🦹 Dangerous Chef (using unwrap()):");
//     // let pizza = make_pizza(false).unwrap(); // This would panic and crash!
//     println!("This would crash the restaurant!\n");
// }

// // 2. The Wise Chef (using match)
// fn wise_chef() {
//     println!("👨🍳 Wise Chef (using match):");
//     match make_pizza(false) {
//         Ok(pizza) => println!("Dinner: {}", pizza),
//         Err(e) => {
//             println!("ERROR: {}", e);
//             println!("Serving backup: {}", make_backup_meal());
//         }
//     }
// }

// // 3. The Practical Chef (using unwrap_or)
// fn practical_chef() {
//     println!("\n👩🍳 Practical Chef (using unwrap_or):");
//     let meal = make_pizza(false).unwrap_or(make_backup_meal());
//     println!("Dinner: {}", meal);
// }

// // 4. The Responsible Chef (using ? operator)
// fn responsible_chef() -> Result<(), String> {
//     println!("\n🧑🍳 Responsible Chef (using ? operator):");
//     let pizza = make_pizza(false)?; // Will return early if error
//     println!("Dinner: {}", pizza);
//     Ok(())
// }

// // 5. The Safe Unwrap (only when absolutely sure)
// fn safe_unwrap_chef() {
//     println!("\n😇 Safe Unwrap Chef (when 100% sure):");
//     // Only unwrap when we KNOW it will succeed
//     let pizza = make_pizza(true).unwrap();
//     println!("Dinner: {}", pizza);
// }

// fn main() {
//     println!("Welcome to the Rusty Kitchen!\n");
    
//     // Dangerous approach (commented out to prevent crashes)
//     dangerous_chef();
    
//     // Safe approaches
//     wise_chef();
//     practical_chef();
    
//     // Handle the error from the ? operator
//     if let Err(e) = responsible_chef() {
//         println!("Error handled in main: {}", e);
//     }
    
//     safe_unwrap_chef();
    
//     println!("\nAll meals served successfully (even with errors)!");
// }
//////////////////////////////////////////////////////////////////////////////////////////////////
// fn main() {
//     // Let's imagine we have a cookie jar
//     let cookie_jar = vec!["🍪", "🍪", "🍪"];
    
//     // We want to take the 5th cookie (but there are only 3)
//     let index = 4; // Trying to access beyond what exists
    
//     // This will panic because we can't get what doesn't exist!
//     let cookie = cookie_jar[index]; // 🚨 PANIC! Index out of bounds
    
//     println!("Yummy cookie: {}", cookie);
// }
////////////////////
// To safely access elements in a vector, we can use two methods:

// fn main() {
//     let cookie_jar = vec!["🍪", "🍪", "🍪"];
//     let index = 4;
    
//     // Safe way 1: Check length first
//     if index < cookie_jar.len() {
//         println!("Safe cookie: {}", cookie_jar[index]);
//     } else {
//         println!("No cookie at position {}! 🚫", index);
//     }
    
//     // Safe way 2: Using get() which returns Option
//     match cookie_jar.get(index) {
//         Some(cookie) => println!("Match got: {}", cookie),
//         None => println!("No cookie there! 👻"),
//     }
// }
///////////////////////////////////////////////////////////

// panic!
// fn make_tea(sugar: u8) {
//     // Sometimes panic is appropriate for unrecoverable errors
//     if sugar > 5 {
//         panic!("🤢 That's too sweet! Health hazard!");
//     }
//     println!("☕ Tea with {} sugars", sugar);
// }

// fn main () {
//     make_tea(4);
// }

//////////////////////////////////////
// {:?}
// use std::fs::File; // 1. We'll try to use the File system
// fn main() {
//     let result = File::open("non_existent.txt"); // 2. Try to open a file that won't exist
//     println!("File open attempt: {}", result); // 3. Print the 'result' (which will be an error) using {:?}
// }
// fn main() {
//     let numbers = vec![10, 20, 30, 40, 50]; // 1. Create a list of numbers (a vector)
//     println!("My list: {}", numbers);       // 2. Print the list using {:?}
//     // 3. The output will show you the exact contents for debugging!
// }
// 1. The Chef's Ingredients and Problems (Enums and Structs)

// First, we define what a 'Meatball' looks like when it's perfect.
// We add #[derive(Debug)] so our chef's magnifying glass can see its details.
// #[derive(Debug)]
// struct CookedMeatball {
//     doneness: String, // e.g., "perfectly cooked"
//     flavor: String,   // e.g., "savory with a hint of garlic"
// }

// // Next, we define the specific problems our chef might encounter.
// // Again, #[derive(Debug)] is crucial for the magnifying glass!
// #[derive(Debug)]
// enum MeatballError {
//     BurntToACrisp { temperature: u16, duration_minutes: u8 },
//     OvenTooCold { current_temp: u16 },
//     ForgotIngredient(String), // e.g., "salt" or "spices"
//     OtherProblem(String),
// }

// // 2. The Oven (Our Baking Function)
// // This function tries to bake the meatballs and returns a Result.
// // It might give us a perfect meatball (Ok) or a detailed error (Err).
// fn bake_meatballs(oven_temp: u16, bake_time_minutes: u8) -> Result<CookedMeatball, MeatballError> {
//     println!("Chef Leo is baking meatballs at {} degrees for {} minutes...", oven_temp, bake_time_minutes);

//     // Simulate different baking outcomes
//     if oven_temp >= 450 && bake_time_minutes >= 50 {
//         // Oops, burnt!
//         Err(MeatballError::BurntToACrisp {
//             temperature: oven_temp,
//             duration_minutes: bake_time_minutes,
//         })
//     } else if oven_temp <= 200 {
//         // Too cold!
//         Err(MeatballError::OvenTooCold { current_temp: oven_temp })
//     } else if oven_temp == 350 && bake_time_minutes == 20 {
//         // Perfect conditions!
//         Ok(CookedMeatball {
//             doneness: "perfectly cooked".to_string(),
//             flavor: "savory with a hint of garlic".to_string(),
//         })
//     } else {
//         // Some other unspecified problem
//         Err(MeatballError::OtherProblem("Unusual baking conditions".to_string()))
//     }
// }

// // 3. Chef Leo's Main Kitchen (main function)
// fn main() {
//     // --- Scenario 1: The Problem! ---
//     println!("\n--- Chef Leo's first attempt (with a mistake) ---");
//     // Chef Leo makes a mistake: oven too hot and too long!
//     let config_result_box = bake_meatballs(500, 60);

//     // Chef Leo uses his special magnifying glass (println! with {:?})
//     // to inspect the 'Result box' content when he's debugging.
//     println!("Chef's Magnifying Glass View: {}", config_result_box);
//     // This will show: Chef's Magnifying Glass View: Err(BurntToACrisp { temperature: 500, duration_minutes: 60 })
//     // "Aha! The oven was 500 degrees for 60 minutes!"

//     // --- Scenario 2: Perfect Meatballs! ---
//     println!("\n--- Chef Leo's second attempt (the fix!) ---");
//     // Chef Leo corrects the recipe!
//     let perfect_meatball_result = bake_meatballs(350, 20);

//     // He still uses the magnifying glass, just to confirm the perfect details.
//     println!("Chef's Magnifying Glass View: {}", perfect_meatball_result);
//     // This will show: Chef's Magnifying Glass View: Ok(CookedMeatball { doneness: "perfectly cooked", flavor: "savory with a hint of garlic" })
//     // "Perfect! Exactly what I wanted!"

//     // --- Scenario 3: Another type of problem ---
//     println!("\n--- Chef Leo's third attempt (oven too cold) ---");
//     let cold_oven_result = bake_meatballs(150, 30);
//     println!("Chef's Magnifying Glass View: {}", cold_oven_result);
//     // This will show: Chef's Magnifying Glass View: Err(OvenTooCold { current_temp: 150 })
// }
// fn calculate_area(length: f64, width: f64) -> f64 {
//     println!("DEBUG: Inside calculate_area with length = {} and width = {}", length, width);
//     let area = length * width;
//     println!("{}", area);
//     area
// }

// fn main() {
//     let l = 10.0;
//     let w = 5.0;
//     let result = calculate_area(l, w);
//     println!("{}", result);
// }

// fn calculate_area(length: f64, width: f64) -> f64 {
//     let area = dbg!(length * width); // Prints 'area' value, file, and line
//     area
// }

// fn main() {
//     let l = 10.0;
//     let w = dbg!(5.0); // Prints 'w' value, file, and line
//     let result = calculate_area(dbg!(l), w); // Prints 'l' value, file, and line
//     println!("Final result: {}", result);
// }
// ///////////////////////////////////////////////////////////////////////////////////////////////////
// unwarp()
// In Rust, unwrap() is a method used to quickly extract the value from a Result or Option type—but it comes with a catch. Here's a breakdown:
// Option<T>
// let some_value = Some(5);
// let x = some_value.unwrap(); // x = 5

// Result<T, E>
// let ok_result: Result<i32, &str> = Ok(42);
// let y = ok_result.unwrap(); // y = 42
// Why Use unwrap()?
// Quick prototyping: Saves time when you’re 100% sure the value is Some or Ok.

// Examples/tests: Used in short snippets where error handling isn’t the focus.

// The Danger
// unwrap() is like saying:
// "Give me the value, and if anything’s wrong, crash the program!" 💥
// let none_value: Option<i32> = None;
// let z = none_value.unwrap(); // ⚠️ Thread panics: "called `Option::unwrap()` on a `None` value"

// Safer Alternatives
// Method	Behavior
// unwrap_or(default)	Returns value or default if None/Err
// unwrap_or_else()	Computes a fallback value lazily (e.g., `unwrap_or_else(		calculate())`)
// ? operator	Propagates errors to the caller (idiomatic Rust)
// Example with unwrap_or:

// rust
// let maybe_num: Option<i32> = None;
// let safe_num = maybe_num.unwrap_or(0); // Returns 0 instead of panicking
