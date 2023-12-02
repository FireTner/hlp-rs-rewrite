use crate::unicount::unicount;
use crate::vec::i8x16;
use crate::layer::*;

// Generates a look up table
// Takes goal as a parameter  
// Returns a vector look up table
//
// Generates a table of all possible unique 1 layer configurations
pub fn gen_lut(goal: i8x16) -> Vec<Layer> {
  let mut layers: Vec<Layer> = Vec::new();
  let goal_uc = unicount(goal);

  for configuration in 0..CONFCOUNT {
    let output = Layer::generate(i8x16::start(), configuration);
    
    // Removes identity function, invalid, illegal and non-unique layers
    if output.layer == i8x16::start() { continue; }
    if output.layer == i8x16::zero() { continue; }
    if unicount(output.layer) < goal_uc { continue; }
    if layers.contains(&output) { continue; }

    layers.push(output);
  }

  return layers;
}