use std::collections::HashSet;
// in this problem the directions are 
// technically opposite
// up as in polar direction of north and so on
// so moving is actually opposite direction of the grid coordinates in the grid
//                                  y, x
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

#[derive(Debug, PartialEq, Eq)]
enum Facing {
	Up,
	Down,
	Left,
	Right
}

#[derive(Debug, PartialEq)]
struct Point {
	y: isize,
	x: isize,
	chr: char,
	facing: Facing,
}
impl Point {
	fn new(y: isize, x: isize, chr: char) -> Self {
		Self { y,x,chr, facing: Facing::Up }
	}

	fn to_string(&self) -> String {
		format!("{},{}",self.y,self.x)
	}

	fn go_forward(&mut self) {
		match self.facing {
			Facing::Up => self.move_up(),
			Facing::Down => self.move_down(),
			Facing::Left => self.move_left(),
			Facing::Right => self.move_right(),
		}
	}

	fn change_direction(&mut self) {
		match self.facing {
			Facing::Up => self.facing = Facing::Right,
			Facing::Down => self.facing = Facing::Left,
			Facing::Left => self.facing = Facing::Up,
			Facing::Right => self.facing = Facing::Down,
		}

	}

	fn move_up(&mut self) {
		self.chr = '^';
		self.y -= 1;
	}
	fn move_down(&mut self) {
		self.chr = 'v';
		self.y += 1;
	}
	fn move_left(&mut self) {
		self.chr = '<';
		self.x -= 1;
	}
	fn move_right(&mut self) {
		self.chr = '>';
		self.x += 1;
	}
}

fn main() {
	// part 1
	let file = std::fs::read_to_string("input").unwrap();
	let mut guard = Point::new(0,0,'^');
	let mut grid = file.lines().enumerate() 
		.map(|(y, l)| {
			if let Some((x, c)) = l.chars()
					.enumerate().find(|(_, c)| *c == '^') 
			{
				// println!("found guard starting point (y-{},x-{}),{}", y,x, c);
				guard = Point::new(y as isize,x as isize,c);
			}

			l.chars().enumerate().map(|(x, c)| {
				if (y == guard.y as usize && x == guard.x as usize) {
					Point::new(y as isize,x as isize,'.')
				} else {
					Point::new(y as isize,x as isize,c)
				}
			}).collect::<Vec<Point>>()
		})
		.collect::<Vec<Vec<Point>>>();

	// dump_grid(&grid, &guard);
	// let mut buf = String::new();

	let mut places_walked = HashSet::new();

	'walking: loop {
		for y in 0..grid.len() {
			for x in 0..grid[y].len() {
				// only check from guard's current spot
				if y == guard.y as usize && x == guard.x as usize {
					if let None = guard_move(y, x, &mut grid, &mut guard) {
						// let _ = std::io::stdin().read_line(&mut buf);
						// println!("moving");
						places_walked.insert(grid[y][x].to_string());
						guard.go_forward();
						// dump_grid(&grid, &guard);
					} else {
						places_walked.insert(grid[y][x].to_string());
						println!("part1 {}", places_walked.len());
						break 'walking;
					}
				}
			}
		}
	}
}

// return return Some(()) if guard reaches obstacle in the grid
// guard always moves "forward" relative to which direction they are facing
fn guard_move(y: usize, x: usize, grid: &mut Vec<Vec<Point>>, guard: &mut Point) -> Option<()> {

	if y + 1 == grid.len() || x + 1 == grid.len() {
		return Some(());
	}

	if guard.facing == Facing::Up && grid[y - 1 as usize][x].chr == '#' {
		guard.change_direction();
		// println!("changing direction");
	}
	// println!("guard coords {},{} direction {}", y,x, guard.chr);
	if guard.facing == Facing::Down && grid[y + 1 as usize][x].chr == '#' {
		guard.change_direction();
		// println!("changing direction");

	}
	if guard.facing == Facing::Left && grid[y][x - 1 as usize].chr == '#' {
		guard.change_direction();
		// println!("changing direction");
	}
	if guard.facing == Facing::Right && grid[y][x + 1 as usize].chr == '#' {
		guard.change_direction();
		// println!("changing direction");
	}

	None
}

fn dump_grid(grid: &Vec<Vec<Point>>, guard: &Point) {
	grid.into_iter().for_each(|(r)| {
		r.into_iter().for_each(|(p)| {
			if p.y == guard.y && p.x == guard.x {
				print!("{} ", guard.chr);
			} else {
				print!("{} ", p.chr);
			}
		});
		println!("");
	});
}
