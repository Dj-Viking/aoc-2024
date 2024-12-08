// #[derive(Clone, Debug)]
// struct Nbor {
//     symbol: String,
//     adj_point: (isize, isize),
// }
// impl Nbor {
//     pub fn new(symbol: String, adj_point: (isize, isize)) -> Self {
//         Self {
//             symbol,
//             adj_point,
//         }
//     }
// }
                              //   y  , x
const UP: (isize, isize) =        ( 1, 0 );
const DOWN: (isize, isize) =      (-1, 0 );
const LEFT: (isize, isize) =      ( 0, -1);
const RIGHT: (isize, isize) =     ( 0, 1 );
const UPLEFT: (isize, isize) =    ( 1, -1);
const DOWNLEFT: (isize, isize) =  (-1, -1);
const UPRIGHT: (isize, isize) =   ( 1, 1 );
const DOWNRIGHT: (isize, isize) = (-1, 1 );

const DIRECTIONS_STRAIGT: [(isize, isize); 4] = [
    UP,
    DOWN,
    RIGHT,
    LEFT
];
const DIRECTIONS_DIAG: [(isize, isize); 4] = [
    UPLEFT,
    DOWNLEFT,
    DOWNRIGHT,
    UPRIGHT
];

const DIRECTIONS: [(isize, isize); 8] = [
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UPLEFT,
    DOWNLEFT,
    DOWNRIGHT,
    UPRIGHT
];

fn add(a: usize, b: isize) -> Option<usize> {
    match b >= 0 {
        true => Some(a + b as usize),
        false => a.checked_sub(-b as usize),
    }
}

fn main() {
    let file = std::fs::read_to_string("sample").unwrap();
    let grid = file.lines().map(str::as_bytes).collect::<Vec<&[u8]>>();

    println!("part1: {}", 
    grid.iter().enumerate() 
        .flat_map(|(y, row)| row.iter().enumerate()
            .filter_map(move |(i, c)| (*c == b'X').then_some((y, i))))
        .fold(0, |acc, coord| acc + 
            DIRECTIONS.iter().fold(0, |acc, dir| {
                let offset = |n: (usize, usize)| -> Option<(usize, usize)> { Some((
                    add(n.0, dir.0).filter(|n| *n < grid.len())?,
                    add(n.1, dir.1).filter(|n| *n < grid[0].len())?
                ))};

                let Some((y, x)) = offset(coord) else { return acc; };
                if grid[y][x] != b'M' { return acc; }
                let Some((y, x)) = offset((y, x)) else { return acc; };
                if grid[y][x] != b'A' { return acc; }
                let Some((y, x)) = offset((y, x)) else { return acc; };
                if grid[y][x] != b'S' { return acc; }
                
                acc + 1
            })));
    // println!("{:?}", coords);
    println!("part2: {}", 
        grid.iter().enumerate() 
        .flat_map(|(y, row)| row.iter().enumerate()
            .filter_map(move |(i, c)| (*c == b'M').then_some((y, i))))
        .fold(0, |acc, coord|
            acc + DIRECTIONS_STRAIGT.iter().fold(0, |acc, dir| {
                let offset = |n: (usize, usize), dir: (isize, isize)| -> Option<(usize, usize)> { Some((
                    add(n.0, dir.0).filter(|n| *n < grid.len())?,
                    add(n.1, dir.1).filter(|n| *n < grid[0].len())?
                ))};

                let offset_check = |n: (usize, usize), dir: (isize, isize), c: u8| -> bool {
                    let Some((y,x)) = offset(coord, dir) else { return false; };
                    grid[y][x] == c
                };

                if !offset_check(coord, (dir.0 * 2, dir.1 * 2), b'M') { return acc; }

                let (s1,s2) = match dir {
                    &RIGHT => match () {
                        _ if offset_check(coord, DOWNRIGHT, b'A') => ((-2, 0), (-2, 2)),
                        _ if offset_check(coord, UPRIGHT, b'A') => ((2, 0), (2, 2)),
                        _ => return acc
                    },
                    &LEFT => match () {
                        _ if offset_check(coord, DOWNLEFT, b'A') => ((-2, 0), (-2, -2)), 
                        _ if offset_check(coord, UPLEFT, b'A') => ((2, 0), (2, -2)),
                        _ => return acc
                    },
                    &DOWN => match () {
                        _ if offset_check(coord, DOWNLEFT, b'A') => ((0, -2), (-2, -2)),
                        _ if offset_check(coord, DOWNRIGHT, b'A') => ((0, 2), (-2, 2)),
                        _ => return acc
                    },
                    &UP => match () {
                        _ if offset_check(coord, UPLEFT, b'A') => ((0, -2), (2, -2)),
                        _ if offset_check(coord, UPRIGHT, b'A') => ((0, 2), (2, 2)),
                        _ => return acc
                    },
                    _ => unreachable!(),
                };

                if !offset_check(coord, s1, b'S') { return acc; }
                if !offset_check(coord, s2, b'S') { return acc; }


                acc + 1
            })  
        )
    );
}
