use std::io;

fn main() {
    let mut string = String::new();
    let _ = io::stdin().read_line(&mut string);
    string = string.replace("\n", "");
    let list: Vec<u128> = string.split(' ').map(|x| x.parse::<u128>().unwrap()).collect();

    let mut index1: usize = if list[0] > list[1] { 0 } else { 1 };
    let mut index2: usize = if list[0] > list[1] { 1 } else { 0 };

    for i in 2..(list.len()) {
        if list[i] >= list[index1] {
            index2 = index1;
            index1 = i;
        } else if list[i] > list[index2] {
            index2 = i;
        }
    }

    println!("{}", list[index1] * list[index2]);
}
