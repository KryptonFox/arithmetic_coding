use bigdecimal::BigDecimal;
use indexmap::IndexMap;
use crate::segment::define_segments;

pub fn arithmetic_decode(
    chars_probability: &IndexMap<char, BigDecimal>,
    p: &BigDecimal,
    q: i64,
    n: i64,
) -> String {
    let mut code = p / BigDecimal::from(2).powi(q);
    let segments = define_segments(chars_probability);

    let mut buf = String::new();
    for _ in 0..n {
        for (char, segment) in segments.iter() {
            if code >= segment.left && code < segment.right {
                buf.push(*char);
                code = (code - &segment.left) / (&segment.right - &segment.left);
                break;
            }
        }
    }
    buf
}
