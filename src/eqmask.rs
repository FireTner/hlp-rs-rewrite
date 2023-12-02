use crate::vec::i8x16;

// Generate eqmask for goal
// Takes goal as parameter
// Returns table of equality masks for goal
//
// Generates equality mask for goal for every index
pub fn gen_eqmask(goal: i8x16) -> [i32; 15] {
  let mut goal_eqmask: [i32; 15] = [0; 15];
  let goal_array = goal.as_array();

  for i in 0..15 {
    let a = i8x16::from_imm(goal_array[i]);
    goal_eqmask[i] = !a.eqmask(&goal);
  }

  return goal_eqmask;
}