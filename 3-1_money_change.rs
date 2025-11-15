fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input = input.replace("\n", "");
    let mut sum = input.parse::<u128>().unwrap();

    let coins = [10, 5, 1];

    let mut count = 0;
    for coin in coins {
        let amount = sum / coin;
        count += amount;
        sum -= coin * amount;
    }
    println!("{}", count);
}
