fn main() {
    println!("Введите строку:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();

    let result = input.rsplit(' ')
        .map(|word| word.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("Результат: {}", result);
}