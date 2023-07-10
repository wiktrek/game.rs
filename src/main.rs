mod functions;
use functions::*;
fn main() {
    println!("main.rs");
    start()
}
fn start() {
    let options = vec!["new game", "load from file"];
    let a = create_dialogue(options.clone(), "Select Option");
    if a == 0 {
        create_file();
    } else {
        read_file();
    }

    println!("You selected option: {} \n{}", a, options[a]);
}
