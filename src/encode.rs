use bigdecimal::BigDecimal;
use indexmap::IndexMap;
use crate::segment::define_segments;

pub fn arithmetic_encode(
    chars_probability: &IndexMap<char, BigDecimal>,
    data: &String,
) -> (BigDecimal, i64) {
    let segments = define_segments(chars_probability);
    let mut left = BigDecimal::from(0);
    let mut right = BigDecimal::from(1);
    for symb in data.chars() {
        let delta = &right - &left;
        right = &left + &delta * &segments.get(&symb).unwrap().right;
        left = &left + &delta * &segments.get(&symb).unwrap().left;
        println!("{symb}: l:{:.80} - r:{:.80}", left, right)
    }
    println!("Границы кода: [{:.80}; {:.80})", left, right);
    let q = q(&left, &right);
    let p = (BigDecimal::from(2).powi(q) * right).with_scale(0);
    (p, q)
}



fn q(left: &BigDecimal, right: &BigDecimal) -> i64 {
    let mut n = BigDecimal::from(1);
    for i in 0..1000 {
        if left + &n < *right {
            return i;
        }
        n = n / BigDecimal::from(2);
    }
    panic!("q большой очень")
}
