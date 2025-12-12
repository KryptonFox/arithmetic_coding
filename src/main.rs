use crate::decode::arithmetic_decode;
use crate::encode::arithmetic_encode;
use bigdecimal::BigDecimal;
use bigdecimal::num_bigint::ToBigInt;
use indexmap::{IndexMap, IndexSet};
use std::io::Write;

mod decode;
mod encode;
mod segment;

fn main() {
    let input = input();

    let chars_probability = calc_probability(&input);
    println!("Таблица вероятностей символов: {:#?}", chars_probability);

    let result = arithmetic_encode(&chars_probability, &input);
    println!(
        "Закодировано!\np: {}\nq: {}\nbinary: {:b}\ncode: {}",
        result.0,
        result.1,
        result.0.to_bigint().unwrap(),
        &result.0 / BigDecimal::from(2).powi(result.1)
    );

    let decoded = arithmetic_decode(&chars_probability, &result.0, result.1, input.chars().count() as i64);
    println!(
        "Декодировано сообщение: {decoded}",

    );
}
fn calc_probability(data: &String) -> IndexMap<char, BigDecimal> {
    let charset: IndexSet<char> = IndexSet::from_iter(data.chars());
    let mut chars_probability: IndexMap<char, BigDecimal> = IndexMap::with_capacity(charset.len());

    for char in charset.iter() {
        let prob = BigDecimal::from(data.chars().filter(|c| c == char).count() as i64)
            / BigDecimal::from(data.chars().count() as i64);
        chars_probability.insert(*char, prob);
    }
    chars_probability.sort_by(|_, p1, _, p2| p1.cmp(p2));

    chars_probability
}

fn input() -> String {
    print!("Введите входной текст: ");
    std::io::stdout().flush().unwrap();

    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).unwrap();

    inp.trim().replace(" ", "").to_uppercase()
}
