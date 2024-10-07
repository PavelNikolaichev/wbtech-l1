use std::io::{self, Write};

#[derive(Debug, Clone)]
struct BigInt {
    digits: Vec<u8>,
    sign: bool,
}

impl BigInt {
    fn new(value: u64) -> Self {
        let mut digits = Vec::new();
        let mut v = value;

        while v > 0 {
            digits.push((v % 10) as u8);
            v /= 10;
        }

        BigInt { digits, sign: true }
    }

    fn from_str(value: &str) -> Self {
        let digits = value
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let sign = value.chars().next().unwrap() == '-';
        BigInt { digits, sign: !sign }
    }

    fn to_string(&self) -> String {
        self.digits
            .iter()
            .rev()
            .map(|&d| (d + b'0') as char)
            .collect()
    }

    fn add(&self, other: &BigInt) -> BigInt {
        let mut result = Vec::new();
        let mut carry = 0;
        let max_len = self.digits.len().max(other.digits.len());

        for i in 0..max_len {
            let a = self.digits.get(i).unwrap_or(&0);
            let b = other.digits.get(i).unwrap_or(&0);
            let sum = a + b + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }

        if carry > 0 {
            result.push(carry);
        }

        BigInt { digits: result, sign: true }
    }

    fn subtract(&self, other: &BigInt) -> BigInt {
        let mut result = Vec::new();
        let mut borrow = 0;

        for i in 0..self.digits.len() {
            let a = self.digits[i];
            let b = other.digits.get(i).unwrap_or(&0) + borrow;
            if a < b {
                result.push(a + 10 - b);
                borrow = 1;
            } else {
                result.push(a - b);
                borrow = 0;
            }
        }

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        BigInt { digits: result, sign: self >= other }
    }

    fn multiply(&self, other: &BigInt) -> BigInt {
        let mut result = vec![0; self.digits.len() + other.digits.len()];

        for (i, &a) in self.digits.iter().enumerate() {
            let mut carry = 0;
            for (j, &b) in other.digits.iter().enumerate() {
                let product = a * b + carry + result[i + j];
                result[i + j] = product % 10;
                carry = product / 10;
            }
            result[i + other.digits.len()] += carry;
        }

        // Удаляем ведущие нули
        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        BigInt { digits: result, sign: self.sign == other.sign }
    }

    fn divide(&self, other: &BigInt) -> BigInt {
        // Простейшая реализация деления с остатком
        let mut remainder = BigInt::new(0);
        let mut result = BigInt::new(0);

        for digit in self.digits.iter().rev() {
            remainder.digits.insert(0, *digit);
            let mut count = 0;

            while remainder >= *other {
                remainder = remainder.subtract(other);
                count += 1;
            }
            result.digits.insert(0, count);
        }

        // Удаляем ведущие нули
        while result.digits.len() > 1 && result.digits.last() == Some(&0) {
            result.digits.pop();
        }

        result
    }
}

impl std::cmp::PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_len = self.digits.len();
        let other_len = other.digits.len();

        if self_len > other_len {
            return Some(std::cmp::Ordering::Greater);
        }
        if self_len < other_len {
            return Some(std::cmp::Ordering::Less);
        }

        for i in (0..self_len).rev() {
            if self.digits[i] > other.digits[i] {
                return Some(std::cmp::Ordering::Greater);
            }
            if self.digits[i] < other.digits[i] {
                return Some(std::cmp::Ordering::Less);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl std::cmp::PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.digits == other.digits
    }
}

fn main() {
    let mut input = String::new();

    print!("Введите первое число (больше 2^20): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let a = BigInt::from_str(input.trim());
    input.clear();

    print!("Введите второе число (больше 2^20): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let b = BigInt::from_str(input.trim());
    input.clear();

    println!("Сложение: {}", a.add(&b).to_string());
    println!("Вычитание: {}", a.subtract(&b).to_string());
    println!("Умножение: {}", a.multiply(&b).to_string());
    println!("Деление: {}", a.divide(&b).to_string());

    // Тесты
    let a = BigInt::from_str("12345678901234567890");

    println!("{}", a.add(&BigInt::from_str("98765432109876543210")).to_string());
    assert_eq!(a.add(&BigInt::from_str("98765432109876543210")).to_string(), "111111111011111111100");
    assert_eq!(a.subtract(&BigInt::from_str("98765432109876543210")).to_string(), "-86419753208641975320");
    assert_eq!(a.multiply(&BigInt::from_str("98765432109876543210")).to_string(), "1219326311126352691002545353284908660");
    assert_eq!(a.divide(&BigInt::from_str("98765432109876543210")).to_string(), "0");

    println!("Тесты пройдены!");
}
