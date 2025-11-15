fn read_input_line_as_vec() -> Vec<String> {
    let mut input = String::new();
    let _ = ::std::io::stdin().read_line(&mut input);
    input = input.replace("\n", "");
    input.split(' ').map(|x| x.to_string()).collect()
}

fn selection_sort(list: &mut Vec<String>) {
    for i in 0..(list.len() - 1) {
        let mut max_index = i;
        for j in (i + 1)..(list.len()) {
            let first = format!("{}{}", list[j], list[max_index]).parse::<u128>().unwrap();
            let second = format!("{}{}", list[max_index], list[j]).parse::<u128>().unwrap();

            if first >= second {
                max_index = j;
            }
        }
        if i != max_index { list.swap(i, max_index); }
    }
}

fn main() {
    let mut _num_input = String::new();
    let _ = ::std::io::stdin().read_line(&mut _num_input);
    let mut list = read_input_line_as_vec();
    selection_sort(&mut list);
    println!("{}", list.join(""));
}
