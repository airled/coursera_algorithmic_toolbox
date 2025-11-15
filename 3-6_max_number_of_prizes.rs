fn main() {
    let mut num_input = String::new();
    let _ = ::std::io::stdin().read_line(&mut num_input);
    num_input = num_input.replace("\n", "");
    let mut num = num_input.parse::<u128>().unwrap();

    let mut i: u128 = 1;
    let mut prizes: Vec<u128> = vec![];
    loop {
        if i <= num {
            prizes.push(i);
            num -= i;
            i += 1;
        } else {
            let last_index = prizes.len() - 1;
            prizes[last_index] += num;
            break;
        }
    }

    let prizes_as_strings: Vec<String> = prizes.into_iter().map(|x| x.to_string()).collect();

    println!("{}", prizes_as_strings.len());
    println!("{}", prizes_as_strings.join(" "));
}
