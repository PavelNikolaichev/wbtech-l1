fn main() {
    println!("Введите строку:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();

    if input.is_empty() {
        println!("Результат: ");
        return;
    }

    let mut l = 0;
    let mut r = input.len() - 1;

    let mut chars: Vec<char> = input.chars().collect();

    while l < r {
        chars.swap(l, r);

        l += 1;
        r -= 1;
    }

    println!("Результат: {}", chars.into_iter().collect::<String>());
}