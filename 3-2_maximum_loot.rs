fn read_input_line_as_vec() -> Vec<u128> {
    let mut input = String::new();
    let _ = ::std::io::stdin().read_line(&mut input);
    input = input.replace("\n", "");
    input.split(' ').map(|x| x.parse::<u128>().unwrap()).collect()
}

fn selection_sort_desc_by_value_per_weight(loot: &mut [(f64, f64, f64)]) {
    for i in 0..(loot.len() - 1) {
        let mut max_index = i;
        for j in (i + 1)..(loot.len()) {
            if loot[j].2 > loot[max_index].2 {
                max_index = j;
            }
        }
        if i != max_index { loot.swap(i, max_index); }
    }
}

fn main() {
    let list = read_input_line_as_vec();
    let num = list[0];
    let max_weight = list[1] as f64;

    let mut loot: Vec<(f64, f64, f64)> = vec![];
    for _i in 0..num {
        let list = read_input_line_as_vec();
        let value = list[0] as f64;
        let weight = list[1] as f64;
        let value_per_weigh = value / weight;
        loot.push((value, weight, value_per_weigh));
    }
    selection_sort_desc_by_value_per_weight(&mut loot);

    let mut weight_sum = 0f64;
    let mut value_sum = 0f64;
    for (_value, weight, value_per_weight) in loot {
        let available_weight = max_weight - weight_sum;
        if available_weight < 1.0 { break; }
        let taken_weight =
            if available_weight >= weight { weight } else { available_weight };

        weight_sum += taken_weight;
        value_sum += value_per_weight * taken_weight;
    }

    println!("{}", value_sum);
}
