pub fn split_with_trim(strr: &String, delim: &str) -> Vec<String> {
    strr.clone()
        .split(delim)
        //trim empty string items
        .filter_map(|x| if !x.to_string().is_empty() { Some(x.to_string()) } else { None })
        .collect::<Vec<String>>()
}

#[derive(Clone, Debug)]
pub struct Nbor {
    symbol: String,
    adj_point: (i32, i32),
}
impl Nbor {
    pub fn new(symbol: String, adj_point: (i32, i32)) -> Self {
        Self {
            symbol: symbol,
            adj_point: adj_point,
        }
    }
}
                          //   x  , y
const UP: (i32, i32) =        (0,   1);
const DOWN: (i32, i32) =      (0,  -1);
const LEFT: (i32, i32) =      (-1,  0);
const RIGHT: (i32, i32) =     (1,   0);
const UPLEFT: (i32, i32) =    (-1,  1);
const DOWNLEFT: (i32, i32) =  (-1, -1);
const UPRIGHT: (i32, i32) =   (1,   1);
const DOWNRIGHT: (i32, i32) = (1,  -1);

pub fn get_nbors(
    grid: &Vec<Vec<String>>,
    curr_x: usize,
    curr_y: usize
) -> Vec<Nbor> {
    let mut vc = vec!();
    let curr_letter = grid[curr_x][curr_y].clone();
    // find out if any direction yeilds a value and then
    // make a neighbor out of it to parse later
    vc.push(Nbor::new("fuck".to_string(), (1,2)));

    vc.clone()
}

fn main() {
    let vc: Vec<i32> = vec!();
    println!("{:?}", vc.get(0));
}
fn main1() {
    let mut grid: Vec<Vec<String>> = vec!();
    let binding = std::fs::read_to_string("sample").unwrap();
    let lines = binding.lines()
        .map(|l| l.to_string()).collect::<Vec<String>>();

    for i in 0..lines.len() {
        grid.push(vec!());
        let splitline = split_with_trim(&lines[i], "");
        for j in 0..splitline.len() {
            grid[i].push(splitline[j].clone());
        }
    }

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let nbors = get_nbors(&grid, x, y); 
        }
    }

}
