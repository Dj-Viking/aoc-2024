use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {

    let stuff: Vec<Vec<usize>> = std::fs::read_to_string("input").expect("this shit to work")
        .lines()
        .map(|x|  x.split("   ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .collect();

    let mut leftlist = vec!();
    let mut rightlist = vec!();
    let mut diffs = vec!();

    let mut hm = HashMap::<usize, usize>::new();

    for i in 0..stuff.len() {
        leftlist.push(stuff[i][0]);

        hm.entry(stuff[i][0]).or_insert(0);

        rightlist.push(stuff[i][1]);
    }

    leftlist.sort();
    rightlist.sort();

    for i in 0..rightlist.len() {
        // println!("{} - {}", leftlist[i], rightlist[i]);
        hm.entry(rightlist[i]).and_modify(|num| *num += 1);
        diffs.push(leftlist[i].abs_diff(rightlist[i]));
    }

    // println!("hm {:?}", hm);
    
    println!("part 1: {}", 
        diffs.iter().sum::<usize>()
    );

    // part 2 - how often the thing in the left appears on the right

    let mut sum: usize = 0;
    for i in 0..leftlist.len() {
        if let Entry::Occupied(out) = hm.entry(leftlist[i]) {
            sum += leftlist[i] * out.get()
        }
    }

    println!("part 2: {}", 
        sum
    );
}
