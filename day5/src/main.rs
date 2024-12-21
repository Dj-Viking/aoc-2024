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




}
