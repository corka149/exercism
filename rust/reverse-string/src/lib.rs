use unicode_segmentation::UnicodeSegmentation;
use std::iter::FromIterator;

pub fn reverse(input: &str) -> String {
    let mut g = UnicodeSegmentation::graphemes(input, true);
    String::from_iter(g.rev())
}
