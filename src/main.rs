// Используем готовый крейт потому что я ленивый.
// На самом деле размер задания при реализации собственного типа BigInt был бы слишком большим, больше чем любое задание в этом курсе.
use num_bigint::BigInt;
use std::str::FromStr;

fn main() {
    // В кач-ве альтернативы существует i128, но допустим что верхней границы не существует.
    let a = BigInt::from_str("123456789012345678901234567890").unwrap();
    let b = BigInt::from_str("987654321098765432109876543210").unwrap();

    println!("{:?} + {:?} = {:?}", a, b, &a + &b);
    println!("{:?} - {:?} = {:?}", a, b, &a - &b);
    println!("{:?} * {:?} = {:?}", a, b, &a * &b);
    println!("{:?} / {:?} = {:?}", a, b, &a / &b);
}