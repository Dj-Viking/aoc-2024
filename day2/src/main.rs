use std::collections::{HashMap};
#[derive(Debug)]
pub struct DirectionChecker {
    increasing: String,
    diff: i32,
}
impl DirectionChecker {
    fn new(increasing: bool, diff: i32) -> Self {
        let mut inc = "";
        if diff == 0 { inc = "false" } 
        else if increasing && diff != 0 { inc = "true" }
        else { inc = "false" };

        Self {
            increasing: inc.to_string(),
            diff: diff,
        }
    }
}
#[derive(Debug)]
pub struct CheckResult {
    safe_levels: bool,
    direction_checks: Vec<DirectionChecker>,
    is_increasing_count: i32,
    is_decreasing_count: i32,
}
impl CheckResult {
    pub fn new(
        safe_levels: bool,
        direction_checks: Vec<DirectionChecker>,
        is_increasing_count: i32,
        is_decreasing_count: i32
    ) -> Self {
        Self {
            safe_levels: safe_levels,
            direction_checks: direction_checks,
            is_increasing_count: is_increasing_count,
            is_decreasing_count: is_decreasing_count,
        }
    }
}
pub fn check_report(report: Vec<i32>) -> CheckResult {

    let mut direction_checks = vec!();
    let mut safe_levels = true;
    let mut is_increasing_count = 0;
    let mut is_decreasing_count = 0;

    'diffcheck: for j in 1..report.len() {
        let prevlevel = report[j - 1];
        let currlevel = report[j];
        
        //println!("levels {} - {}", prevlevel, currlevel);
        //println!("levels diff {:?}", prevlevel.abs_diff(currlevel));
        let diff = prevlevel.abs_diff(currlevel);

        if diff < 1
            || diff > 3
        {
            // println!("================================");
            // println!("not safe level difference  {} {}", prevlevel, currlevel);
            safe_levels = false;
            break 'diffcheck;
        } 
        // else { println!("SAFE LEVELSS"); }
    }

    for j in 1..report.len() {
        let prevlevel = report[j - 1];
        let currlevel = report[j];
         
        direction_checks.push(DirectionChecker::new(
            prevlevel - currlevel < 0,
            prevlevel.abs_diff(currlevel) as i32
        ));
    }
    // println!("{:?}", dirchecks);

    for d in 0..direction_checks.len() {
        let dir = &direction_checks[d];
        if dir.increasing == "true" {
            is_increasing_count += 1;
        } else { is_decreasing_count += 1; }
    }
    
    CheckResult::new( 
        safe_levels,
        direction_checks,
        is_increasing_count,
        is_decreasing_count,
    )
}

pub fn part1() {
    let reportslist: Vec<Vec<i32>> = std::fs::read_to_string("input").expect("the file to open lol")
        .lines()
        .map(|l| l.split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        )
        .collect();
    // println!("{:?}", reportslist);

    let mut safe = 0;
    for i in 0..reportslist.len() {
        let report = reportslist[i].clone();
        let check_result = check_report(report.clone());
        //println!("checkresult {:?}", check_result);

        // part1 rules
        if check_result.safe_levels 
        && (
            check_result.is_increasing_count
              == check_result.direction_checks.len() as i32 
            || check_result.is_decreasing_count 
              == check_result.direction_checks.len() as i32
        ) { safe += 1 }

    }

    println!("part1: {}", safe);
}
pub fn part2() {
    let reportslist: Vec<Vec<i32>> = std::fs::read_to_string("input").expect("the file to open lol")
        .lines()
        .map(|l| l.split(" ").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();
    // println!("{:?}", reportslist);

    let mut safe = 0;
    let mut unsafereports: Vec<Vec<i32>> = vec!();
    for i in 0..reportslist.len() {
        let report = reportslist[i].clone();
        let check_result = check_report(report.clone());
        // println!(" report =------> {:?}", report);

        if check_result.safe_levels 
        && (
            check_result.is_increasing_count
              == check_result.direction_checks.len() as i32 
            || check_result.is_decreasing_count 
              == check_result.direction_checks.len() as i32
        ) { safe += 1 }
        else {
            // add into unsafe report list
            // and re-eval by removing levels until its safe
            // if it can be
            unsafereports.push(report)
        }

    }
    // run checks again each check will have one less item
    // 0th taken out check - 1st taken out check - etc...
    for i in 0..unsafereports.len() {
        let ureport = unsafereports[i].clone();
        let mut removeditemreport = ureport.clone();
        

        //println!("removedsomething {:?}", removeditemreport);
        let mut j = 0;
        let rrclone = removeditemreport.clone();
        'removed: for k in 0..rrclone.len() {
            removeditemreport = rrclone.clone();
            removeditemreport.remove(j);

            //println!("rr {:?} {}", removeditemreport, k);


            let check_result = check_report(removeditemreport.clone());
            //println!("res {:?}", check_result);
            // is it safe now? after removing the item?
            if check_result.safe_levels 
            && (
                check_result.is_increasing_count
                  == check_result.direction_checks.len() as i32 
                || check_result.is_decreasing_count 
                  == check_result.direction_checks.len() as i32
            ) { 
                //println!("safe now after removing level");
                safe += 1;
                break 'removed;
            } 
            //else { println!("still not safe!!!"); }

            j += 1;

        }

    }

    println!("part2: {}", safe);
}

fn main() {
    part1();
    part2();
}
