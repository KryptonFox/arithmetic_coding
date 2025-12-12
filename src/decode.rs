use bigdecimal::BigDecimal;
use indexmap::IndexMap;

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
        for i in 0..segments.len() {
            if code >= segments[i].left && code < segments[i].right {
                buf.push(segments[i].character);
                code = (code - &segments[i].left) / (&segments[i].right - &segments[i].left);
                break;
            }
        }
    }
    buf
}

struct Segment {
    left: BigDecimal,
    right: BigDecimal,
    character: char,
}

fn define_segments(chars_probability: &IndexMap<char, BigDecimal>) -> Vec<Segment> {
    let mut segments: Vec<Segment> =
        Vec::with_capacity(chars_probability.len());
    let mut l = BigDecimal::from(0);
    for (char, prob) in chars_probability.iter() {
        let segment = Segment {
            left: l.clone(),
            right: &l + prob,
            character: *char,
        };
        segments.push(segment);
        l += prob;
    }
    segments
}
