fn pisano(m: u128) -> u128{
    let mut i: u128 = 0;
    let mut previous: u128 = 0;
    let mut current: u128 = 1;

    loop {
        let temp = current;
        current = (current + previous) % m;
        previous = temp;
        i += 1;

        if previous == 0 && current == 1 {
            return i;
        }
    }
}

fn fib(num: u128, modulo: u128) -> u128 {
    let mut previous: u128 = 0;
    let mut current: u128 = 1;

    for _ in 0..num {
        let temp = current;
        current = (previous + current) % modulo;
        previous = temp;
    }
    return previous;
}

fn main() {
    let mut input = String::new();
    let _ = ::std::io::stdin().read_line(&mut input);
    input = input.replace("\n", "");
    let input_nums: Vec<u128> =
        input.split(' ').map(|x| x.parse::<u128>().unwrap()).collect();
    let num: u128 = input_nums[0];
    let modulo: u128 = input_nums[1];

    let pisano_cycle = pisano(modulo);
    let reduced_num = num % pisano_cycle;

    println!("{}", fib(reduced_num, modulo));
}
