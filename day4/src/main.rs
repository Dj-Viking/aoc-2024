const UP: (isize, isize) =        ( 1, 0 );
const DOWN: (isize, isize) =      (-1, 0 );
const LEFT: (isize, isize) =      ( 0, -1);
const RIGHT: (isize, isize) =     ( 0, 1 );
const UPLEFT: (isize, isize) =    ( 1, -1);
const DOWNLEFT: (isize, isize) =  (-1, -1);
const UPRIGHT: (isize, isize) =   ( 1, 1 );
const DOWNRIGHT: (isize, isize) = (-1, 1 );

const DIRECTIONS_STRAIGHT: [(isize, isize); 4] = [
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

fn offset(grid: &Vec<&[u8]>, n: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> { Some((
    add(n.0, dir.0).filter(|n| *n < grid.len())?,
    add(n.1, dir.1).filter(|n| *n < grid[0].len())?
))}

fn offset_check(grid: &Vec<&[u8]>, coord: (usize, usize), dir: (isize, isize), c: u8) -> bool {
    let Some((y,x)) = offset(grid, coord, dir) else { return false; };
    grid[y][x] == c
}

fn main() {
    let file = std::fs::read_to_string("input").unwrap();
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
            .filter_map(move |(i, c)| (*c == b'A').then_some((y, i))))
        .fold(0, |acc, coord| {

            // if there isn't 2 M's diagonally from the A in left right up or down directions
            // then continue
            //
            // M M | S S | M S | S M
            //  A  |  A  |  A  |  A
            // S S | M M | M S | S M
             
            // what were the two M coords? use those to check where the S's should be
            // opposite side of the M should be S
            if offset_check(&grid, coord, UPRIGHT, b'M')
            && offset_check(&grid, coord, UPLEFT, b'M')
            && offset_check(&grid, coord, DOWNRIGHT, b'S') 
            && offset_check(&grid, coord, DOWNLEFT, b'S')
                { return acc + 1; }

            if offset_check(&grid, coord, DOWNRIGHT, b'M')
            && offset_check(&grid, coord, DOWNLEFT, b'M')
            && offset_check(&grid, coord, UPRIGHT, b'S') 
            && offset_check(&grid, coord, UPLEFT, b'S')
                { return acc + 1; }

            if offset_check(&grid, coord, UPLEFT, b'M')
            && offset_check(&grid, coord, DOWNLEFT, b'M')
            && offset_check(&grid, coord, UPRIGHT, b'S') 
            && offset_check(&grid, coord, DOWNRIGHT, b'S')
                { return acc + 1; }

            if offset_check(&grid, coord, UPRIGHT, b'M')
            && offset_check(&grid, coord, DOWNRIGHT, b'M')
            && offset_check(&grid, coord, UPLEFT, b'S') 
            && offset_check(&grid, coord, DOWNLEFT, b'S')
                { return acc + 1; }
            acc 
        })
    );
}
