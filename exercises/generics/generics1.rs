// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new(); // Apparently vec is generic and so we have to specify what it is
    shopping_list.push("milk");
}
