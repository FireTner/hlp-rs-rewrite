use crate::vec::i8x16;

pub fn apply_layer(input: &i8x16, mask: &i8x16, _goal: &i8x16) -> i8x16 {
  return mask.shuffle(input);
}