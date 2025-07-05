use num_bigint::BigUint;
use std::str::FromStr;

fn main() {
    sample();
}

fn sample() {
    // 文字列から初期化（超巨大数もOK）
    let a = BigUint::from_str("123456789012345678901234567890").unwrap();
    let b = BigUint::from(987654321u64);

    // 足し算・引き算・掛け算・割り算
    let sum = &a + &b;
    let product = &a * &b;

    println!("sum     = {}", sum);
    println!("product = {}", product);
}
