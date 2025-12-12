use bigdecimal::BigDecimal;
use indexmap::IndexMap;

pub struct Segment {
    pub left: BigDecimal,
    pub right: BigDecimal,
}

pub fn define_segments(chars_probability: &IndexMap<char, BigDecimal>) -> IndexMap<char, Segment> {
    let mut segments: IndexMap<char, Segment> = IndexMap::with_capacity(chars_probability.len());
    let mut l = BigDecimal::from(0);

    for (char, prob) in chars_probability.iter() {
        let segment = Segment {
            left: l.clone(),
            right: l.clone() + prob,
        };
        segments.insert(*char, segment);
        l += prob;
    }
    segments
}