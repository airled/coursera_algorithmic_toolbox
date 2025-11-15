fn read_input_line_as_vec() -> Vec<u128> {
    let mut input = String::new();
    let _ = ::std::io::stdin().read_line(&mut input);
    input = input.replace("\n", "");
    input.split(' ').map(|x| x.parse::<u128>().unwrap()).collect()
}

fn selection_sort(list: &mut Vec<u128>) {
    for i in 0..(list.len() - 1) {
        let mut max_index = i;
        for j in (i + 1)..(list.len()) {
            if list[j] > list[max_index] {
                max_index = j;
            }
        }
        if i != max_index { list.swap(i, max_index); }
    }
}

fn dot_product(vec1: Vec<u128>, vec2: Vec<u128>) -> u128 {
    let mut result: u128 = 0;
    for i in 0..(vec1.len()) {
        result += vec1[i] * vec2[i];
    }
    result
}

fn main() {
    let mut _num_input = String::new();
    let _ = ::std::io::stdin().read_line(&mut _num_input);

    let mut prices = read_input_line_as_vec();
    let mut clicks = read_input_line_as_vec();

    selection_sort(&mut prices);
    selection_sort(&mut clicks);

    println!("{}", dot_product(prices, clicks));
}
