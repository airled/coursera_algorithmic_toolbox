use std::io;

fn main() {
	let mut string = String::new();
	let _ = io::stdin().read_line(&mut string);
	string = string.replace("\n", "");
	let list: Vec<u128> = string.split(' ').map(|x| x.parse::<u128>().unwrap()).collect();

	print!("{}", list[0] + list[1]);
}
