fn main() {
    // We create an empty backpack (Vec) for our fruit.
    // collection of things that can change
    // Notice how we don't say how many fruits it will hold yet.
    let mut my_fruit_backpack = Vec::new(); // 'mut' means it can change!

    println!("At the start: {:?}", my_fruit_backpack); // It's empty!

    // Time for snack break! Let's add some fruit.
    my_fruit_backpack.push("Apple"); // We add an "Apple"
    my_fruit_backpack.push("Banana"); // Then a "Banana"
    my_fruit_backpack.push("Orange"); // And an "Orange"

    println!("After adding fruit: {:?}", my_fruit_backpack); // Now it has 3 fruits!

    // Oh no, you ate the banana! Let's take it out.
    // This is a bit more complex in real code, but conceptually, you remove it.
    // For simplicity, let's just say you took the last one out for now.
    let eaten_fruit = my_fruit_backpack.pop(); // .pop() removes the last item

    println!("After eating a fruit: {:?}", my_fruit_backpack); // Now it only has 2 fruits!
    println!("You ate the: {:?}", eaten_fruit); // Which one did you eat?

    // Later, your friend gives you a grape!
    my_fruit_backpack.push("Grape");

    println!("After getting more fruit: {:?}", my_fruit_backpack); // Your backpack got bigger again!
} 


//                                                              ARRAY                                 VEC
// Size	                                           Fixed (known at compile time)	   Dynamic (can grow/shrink at runtime)
// Memory                                                         	Usually Stack	       Heap for data, Stack for Vec handle
// Flexibility	                                    Rigid, cannot add/remove elements 	   Flexible, can push, pop, insert, remove
// Syntax	                                        [value1, value2, ...] or [T; size]	   Vec::new(), vec![] macro
// Best Use Case	                            Fixed collections, performance-critical	   Collections whose size changes, general-purpose lists
