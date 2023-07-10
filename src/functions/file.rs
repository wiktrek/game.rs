use std::fs;
pub fn create_file() {
    fs::write("./data.json", "[{\"name\":\"test\"}]").expect("error writing file")
}
pub fn read_file() {
    let data = fs::read_to_string("./data.json").expect("error reading file");
    println!("{}", data);
}
