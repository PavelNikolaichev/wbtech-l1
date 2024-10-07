fn main() {
    let mut strings: Vec<String> = Vec::new();

    println!("Введите кол-во строк:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    for i in 0..n {
        input.clear();
        println!("Введите строку {}:", i+1);
        std::io::stdin().read_line(&mut input).unwrap();

        // O(n) поиск, с хеш-таблицей было бы O(1) :(
        if !strings.contains(&input.trim().to_string()) {
            strings.push(input.trim().to_string());
        }
    }

    println!("Уникальные строки: \n{}", strings.join("\n"));
}