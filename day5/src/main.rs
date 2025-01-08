fn parse_rules_and_updates(file: &str) 
    -> (Vec<(usize,usize)>,Vec<Vec<usize>>) {

    let (rules,updatelist) = file.split_once("\n\n").unwrap();

    let rules = rules.lines()
        .map(|s| s.split_once('|')
            .map(|(l,r)| (l.parse::<usize>().unwrap(),r.parse::<usize>().unwrap())).unwrap())
        .collect::<Vec<(usize,usize)>>();

    // add each right side as a key and then the left is value for the ordering rule_nums

    let update_nums = updatelist.lines()
        .map(|s| s.split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<Vec<usize>>>();

    (rules,update_nums)
}
fn main() {
    let file = std::fs::read_to_string("input")
        .unwrap();

    let (rules, update_nums) = parse_rules_and_updates(&file);

	// for testing
    // let update_nums = vec![vec![0,1,2,3,4,5]];
    // let rules = vec![(0,1), (1,2), (2,3), (3,4), (4,5)];
	//
    let matched_rules_for_num = |n| (
        rules.iter().filter(|(l,_)| *l == n).collect::<Vec<&(usize,usize)>>(),
        rules.iter().filter(|(_,r)| *r == n).collect::<Vec<&(usize,usize)>>());

    let ulen = update_nums.len();
    let answer = update_nums.into_iter()
        .fold(Vec::with_capacity(ulen), |mut acc, list| { 
            if list.windows(3).all(|win| {
                let (prev, next) = matched_rules_for_num(win[1]);
                if !next.iter().any(|(n, _)| *n == win[0]) { return false; }
                prev.iter().any(|(_, n)| *n == win[2])
            }) { acc.push(list); } 
        acc })
        .into_iter()
        .map(|l| unsafe { *l.get_unchecked(l.len() / 2) })
        .sum::<usize>();

    println!("{:?}", answer);

}
