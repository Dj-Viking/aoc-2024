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

#[derive(Debug)]
enum Facing {
	Up,
	Down,
	Left,
	Right
}

#[derive(Debug)]
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

	fn do_move(&mut self) {
		match self.facing {
			Facing::Up => self.move_up(),
			Facing::Down => self.move_down(),
			Facing::Left => self.move_left(),
			Facing::Right => self.move_right(),
		}
	}

	fn move_up(&mut self) {
		self.y -= UP.0;
	}
	fn move_down(&mut self) {
		self.y += DOWN.0;
	}
	fn move_left(&mut self) {
		self.x -= LEFT.1;
	}
	fn move_right(&mut self) {
		self.x -= RIGHT.1;
	}
}

fn main() {
	let file = std::fs::read_to_string("sample").unwrap();
	let mut guard = Point::new(0,0,'^');
	let mut grid = file.lines().enumerate() 
		.map(|(y, l)| {
			if let Some((x, c)) = l.chars()
					.enumerate().find(|(_, c)| *c == '^') 
			{
				println!("found guard starting point (y-{},x-{}),{}", y,x, c);
				guard = Point::new(y as isize,x as isize,c);

			}

			l.chars().enumerate().map(|(x, c)| {
					if (y == guard.y as usize && x == guard.x as usize) {
						Point::new(y as isize,x as isize,'.')
					} else {
						Point::new(y as isize,x as isize,c)
					}
				}
			).collect::<Vec<Point>>()
		})
		.collect::<Vec<Vec<Point>>>();

	dump_grid(&grid, &guard);
	let mut buf = String::new();

	'walking: loop {
		if let None = guard_move(&mut grid, &guard) {
			let _ = std::io::stdin().read_line(&mut buf);
			println!("moving");
			guard.do_move();
			dump_grid(&grid, &guard);
		} else { 
			// change guard direction?
		}
	}
}

// return return Some(()) if guard reaches obstacle in the grid
// guard always moves "forward" relative to which direction they are facing
fn guard_move(grid: &mut Vec<Vec<Point>>, guard: &Point) -> Option<()> {
	None
}

fn dump_grid(grid: &Vec<Vec<Point>>, guard: &Point) {
	grid.into_iter().for_each(|(r)| {
		r.into_iter().for_each(|(p)| {
			if (p.y == guard.y && p.x == guard.x) {
				print!("{} ", guard.chr);
			} else {
				print!("{} ", p.chr);
			}
		});
		println!("");
	});
}
