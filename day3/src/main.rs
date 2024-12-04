use regex::Regex;

pub fn part1() {
    let binding = std::fs::read_to_string("input").unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let mut sum = 0;

    let rgx = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))")
        .unwrap(); 

    //println!("{:?}", lines[0]);
    
    let numrgx = Regex::new(r"\d{1,3},\d{1,3}")
        .unwrap();

    let mut stuff_to_eval: Vec<String> = vec!();
    for i in 0..lines.len() {
        
        stuff_to_eval = rgx
            .find_iter(lines[i])
            .map(|f| f.as_str().to_string()).collect();
         
        for i in 0..stuff_to_eval.len() {
            let expr = &stuff_to_eval[i];
            let nums: &str = numrgx.find_iter(expr)
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
pub fn part2() {
    let binding = std::fs::read_to_string("input").unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let mut sum = 0;

    let rgx = Regex::new(r"(do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\))")
        .unwrap(); 

    //println!("{:?}", lines[0]);
    
    let numrgx = Regex::new(r"\d{1,3},\d{1,3}")
        .unwrap();

    let mut should_eval = true;

    for i in 0..lines.len() {

        let stuff_to_eval: Vec<String> = rgx
            .find_iter(lines[i])
            .map(|f| f.as_str().to_string()).collect();
        //println!("stuff {:?}", stuff_to_eval);

        for j in 0..stuff_to_eval.len() {
            if stuff_to_eval[j] == "don't()" {
                should_eval = false; 
                continue; 
            }
            if stuff_to_eval[j] == "do()" {
                should_eval = true;
                continue;
            }
            if should_eval {
                let expr = &stuff_to_eval[j];
                let nums: &str = numrgx.find_iter(expr)
                    .map(|f| f.as_str()).collect::<Vec<&str>>()[0];
                let pair: Vec<i32> = nums.split(",")
                    .map(|s| s.parse::<i32>().unwrap()).collect();

                //println!("pair {:?}", pair);
                sum += pair[0] * pair[1];
            }
        }
    }
    
    println!("part 2: {}", sum);
}
fn main() {
    part1();
    part2();
}
