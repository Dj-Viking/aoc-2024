use regex::Regex;

fn main() {
    let binding = std::fs::read_to_string("sample").unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let rgx = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();

    println!("{:?}", lines);
    for i in 0..lines.len() {

    }
}
