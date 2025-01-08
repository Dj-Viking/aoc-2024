fn guard_move() {

}

fn dump_grid(grid: &Vec<Vec<char>>) {
	grid.into_iter().for_each(|r| {
		r.into_iter().for_each(|c| {
			print!("{}", c);
		});
		println!("");
	});
}

fn main() {
	let file = std::fs::read_to_string("sample").unwrap();
	let grid = file.lines() 
		.map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

	dump_grid(&grid);

}
