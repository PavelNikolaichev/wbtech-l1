use std::io::{self};

fn set_bit(mut num: i64, bit_position: usize, value: u8) -> i64 {
    if bit_position > 63 {
        panic!("Bit position must be between 0 and 63");
    }

    match value {
        1 => num |= 1 << bit_position,
        0 => num &= !(1 << bit_position),
        _ => panic!("Value must be 0 or 1"),
    }

    num
}

fn main() {
    println!("Введите число: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let num: i64 = input.trim().parse().unwrap();

    println!("Введите позицию бита (0-63): ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let bit_position: usize = input.trim().parse().unwrap();

    println!("Введите значение бита (0 или 1): ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let value: u8 = input.trim().parse().unwrap();

    let result = set_bit(num, bit_position, value);
    println!("Результат: {}", result);
}
