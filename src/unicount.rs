use crate::vec::i8x16;
use itertools::Itertools;

// Counts the number of unique elements
// Takes input as parameters
// Returns the number of unique elements
pub fn unicount(input: i8x16) -> usize {
  input.as_array().iter().unique().count()
}