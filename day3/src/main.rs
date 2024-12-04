use regex::Regex;

fn main() {
    let binding = std::fs::read_to_string("input").unwrap();
    let mut sum = 0;
    let lines: Vec<&str> = binding.lines().collect();
    let rgx = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))")
        .unwrap();

    let mut stuff_to_eval = vec!();
    //println!("{:?}", lines[0]);
    
    let rgx2 = Regex::new(r"\d{1,3},\d{1,3}")
        .unwrap();

    for i in 0..lines.len() {
        stuff_to_eval = rgx
            .find_iter(lines[i])
            .map(|f| f.as_str().to_string()).collect();
         
        for i in 0..stuff_to_eval.len() {
            let expr = &stuff_to_eval[i];
            let nums: &str = rgx2.find_iter(expr)
                .map(|f| f.as_str()).collect::<Vec<&str>>()[0];
            let pair: Vec<i32> = nums.split(",")
                .map(|s| s.parse::<i32>().unwrap()).collect();

            //println!("pair {:?}", pair);
            sum += pair[0] * pair[1];
        }
    }
    // 34806505 too low??
    // lol it needed to be nested :)
    // 182619815
    println!("part 1: {}", sum);
}
