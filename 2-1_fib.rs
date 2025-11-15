use std::io;

fn main() {
    let mut num_input = String::new();
    io::stdin().read_line(&mut num_input);
    num_input = num_input.replace("\n", "");

    let num = num_input.parse::<u64>().unwrap();

    let mut previous: u128 = 0;
    let mut current: u128 = 1;

    for _ in 0..num {
        let temp = current;
        current = previous + current;
        previous = temp;
    }

    println!("{}", previous);
}
