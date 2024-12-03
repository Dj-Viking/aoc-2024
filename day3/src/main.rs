fn main() {
    let binding = std::fs::read_to_string("input").unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    println!("{:?}", lines);
}
