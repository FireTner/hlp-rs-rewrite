use crate::unicount::unicount;
use crate::layer::Layer;
use crate::vec::i8x16;

// Generates a table of possible pairs
// Takes look up table and goal as a parameter  
// Returns a a vector of vectors that store indicies to look up table and unique functions
//
// Generates a vector of vectors that contains possible indicies to the look up table
//  for next function such that no non unique, invalid, and illegal indicies should occur
pub fn gen_pairs(layers: &Vec<Layer>, goal: i8x16) -> (Vec<Vec<usize>>, usize) {
  let mut pairs: Vec<Vec<usize>> = vec![Vec::new(); layers.len() + 1];
  let mut tmp: Vec<i8x16> = Vec::new();
  let start: i8x16 = i8x16::start();
  let goal_uc = unicount(goal);

  for i in 0..layers.len() {
    // Output of first layer
    let output = layers[i].layer;

    for j in 0..layers.len() {
      // Output of next layer
      let output2 = i8x16::shuffle(&layers[j].layer, &output);

      // Removes identity function, invalid, illegal and non-unique layers
      if output2 == start { continue; }
      if output2 == output { continue; }
      if unicount(output2) < goal_uc { continue; }
      if layers.contains( &Layer { layer: output2, layerconf: 0 }) { continue; }
      if tmp.contains(&output2) { continue; }

      tmp.push(output2);
      pairs[i].push(j);
    }
  }

  return (pairs, tmp.len());
}