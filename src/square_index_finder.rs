// pub struct Square {
//     index: SquareIndex,
// }

use std::ops::Index;

use crate::chess_board::SquareIndex;

// impl SquareIndexFinder {
pub fn from_notation(notation: &str) -> Option<SquareIndex> {
    let files = "abcdefgh";
    let ranks = "12345678";

    let file = notation.chars().nth(0);
    let rank = notation.chars().nth(1);

    let file_ix = files.find(file?);
    let rank_ix = ranks.find(rank?);

    let ix = Some((file_ix? + rank_ix? * 8).try_into().unwrap());

    return ix;
}

pub fn to_notation(index: SquareIndex) -> String {
    return "A1".to_string();
}

pub fn from_file_and_rank(file: SquareIndex, rank: SquareIndex) -> SquareIndex {
    return rank * 8 + file;
}
// }

#[cfg(test)]
// mod square_index_finder_tests {
#[test]
fn test_from_notation() {
    assert_eq!(from_notation("a1"), Some(0));
    assert_eq!(from_notation("a8"), Some(56));
    assert_eq!(from_notation("h1"), Some(7));
    assert_eq!(from_notation("h8"), Some(63));
    assert_eq!(from_notation("dan"), None);
    assert_eq!(from_notation("123"), None);
    assert_eq!(from_notation(""), None);
}
// }
