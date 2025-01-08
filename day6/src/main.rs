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

// return return Some(()) if guard reaches obstacle in the grid
fn guard_move(grid: &Vec<Vec<char>>) -> Option<()> {
	return Some(());
}

fn dump_grid(grid: &Vec<Vec<char>>) {
	grid.into_iter().for_each(|r| {
		r.into_iter().for_each(|c| {
			print!("{} ", c);
		});
		println!("");
	});
}

fn main() {
	let file = std::fs::read_to_string("sample").unwrap();
	let grid = file.lines() 
		.map(|l| 
			l.chars().collect::<Vec<char>>())
		.collect::<Vec<Vec<char>>>();

	dump_grid(&grid);
	let mut buf = String::new();


	'walking: loop {
		while let Some(()) = guard_move(&grid) {
			let _ = std::io::stdin().read_line(&mut buf);
			println!("moving");
		}
	}
}
