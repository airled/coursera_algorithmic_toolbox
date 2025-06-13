use std::io;

fn main() {
	let mut string = String::new();
	let _ = io::stdin().read_line(&mut string);
	string = string.replace("\n", "");
	let list: Vec<u128> = string.split(' ').map(|x| x.parse::<u128>().unwrap()).collect();

	let first_num = list[0];
	let second_num = list[1];

	let mut first: u128;
	let mut second: u128;

	if first_num >= second_num {
		first = first_num;
		second = second_num;
	} else {
		first = second_num;
		second = first_num;
	}

	while second != 0 {
		let modulo = first % second;
		first = second;
		second = modulo;
	}

	println!("{}", first);
}
