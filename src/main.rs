use crate::functions::create_dialogue;

mod functions;
// use functions::*;
fn main() {
    let options = vec!["new game", "load from file"];
    let a = create_dialogue(options, "Select Option");

    println!("main.rs");
    println!("You selected: {}", a)
}
