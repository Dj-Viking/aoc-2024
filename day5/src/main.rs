fn parse_rules_and_updates(
    file: &String
) -> (Vec<Vec<usize>>,Vec<Vec<usize>>) 
{
    let ordering_rules = file.lines()
        .filter(|l| !l.contains(',') && !l.is_empty())
        .map(|s| 
            s.split('|').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let ordering_rules_nums = ordering_rules.into_iter()
        .map(|x| 
            x.into_iter()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let update_numstrs = file.lines()
        .filter(|l| l.contains(',') && !l.is_empty())
        .map(|s|
            s.split(',').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let update_nums = update_numstrs.into_iter()
        .map(|x|
            x.into_iter()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    (ordering_rules_nums,update_nums)
}
fn main() {
    let file = std::fs::read_to_string("sample")
        .unwrap();

    let (ordering_rules_nums,update_nums) = parse_rules_and_updates(&file);


    // which one of the updates lists
    // is already in the right order?
    
    let mut updates_in_correct_order = Vec::<Vec<usize>>::new();

    for update in &update_nums 
    {
        for rule_nums in &ordering_rules_nums
        {
            let (first,second) = (rule_nums[0], rule_nums[1]);

            // if the number in the rule is not in the update list
            // then go to the next update numlist
            if !update.contains(&first) || !update.contains(&second) {
                continue;
            }

            println!("update {:?} \n\t\t\tfor rule -> {},{}", 
                update, first, second);

        }
        println!("=======================================================");
        break;
    }

}
