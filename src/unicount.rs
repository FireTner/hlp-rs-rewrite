use crate::vec::i8x16;
use itertools::Itertools;

pub fn unicount(value: i8x16) -> usize {
  value.as_array().iter().unique().count()
}