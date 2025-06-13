use std::io;

fn gcd(x: u128, y: u128) -> u128 {
	let mut first: u128 = x;
	let mut second: u128 = y;

	if x < y {
		first = y;
		second = x;
	}

	while second != 0 {
		let modulo = first % second;
		first = second;
		second = modulo;
	}

	return first;
}

fn main() {
	let mut string = String::new();
	let _ = io::stdin().read_line(&mut string);
	string = string.replace("\n", "");
	let list: Vec<u128> = string.split(' ').map(|x| x.parse::<u128>().unwrap()).collect();
	let first = list[0];
	let second = list[1];
	let lcm = first * second / (gcd(list[0], list[1]));

	print!("{}", lcm);
}
