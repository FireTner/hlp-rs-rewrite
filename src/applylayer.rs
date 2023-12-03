use crate::tables::Tables;
use crate::vec::i8x16;

// Applies transmutation to a layer
// Takes input to transmute, mask to transmute by, and goal as parameters
// Returns the transmuted i8x16
#[inline(always)]
pub fn apply_layer(input: &i8x16, mask: &i8x16, tables: &mut Tables) -> i8x16 {
  let result = mask.shuffle(input);
  let result_array = result.as_array();

  for i in 0..15 {
    let a = i8x16::from_imm(result_array[i]);
    let b = a.eqmask(&result);
    
    let c = tables.goal_eqmask[i];

    if (b & c) != 0 { return i8x16::zero(); }
  }

  return result;
}