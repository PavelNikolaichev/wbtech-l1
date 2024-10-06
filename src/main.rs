// Используем BTree для отсортированной хэш-таблицы.
use std::collections::BTreeMap;

fn group_temperatures(temps: Vec<f64>) -> BTreeMap<String, Vec<f64>> {
    let mut groups: BTreeMap<String, Vec<f64>> = BTreeMap::new();

    for &temp in &temps {
        let range_start = (temp / 10.0).floor() * 10.0;

        // Не самый оптимальный способ, технически можно использовать подход bucket sort,
        // сделав в кач-ве ключа просто число десятков. Зато так красиво выводится)
        let range_key = format!("[{:.1}, {:.1})", (temp / 10.0).floor() * 10.0, range_start + 10.0);

        groups.entry(range_key).or_insert(Vec::new()).push(temp);
    }

    groups
}

fn main() {
    let temps = vec![
        -25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5
    ];

    let grouped = group_temperatures(temps);

    for (range, values) in grouped {
        println!("{}: {:?}", range, values);
    }
}
