use crate::unicount::unicount;
use crate::layer::Layer;
use crate::vec::i8x16;

pub fn gen_pairs(layers: &Vec<Layer>, goal: i8x16) -> (Vec<Vec<usize>>, usize) {
  let start: i8x16 = i8x16::from_array(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
  let mut pairs: Vec<Vec<usize>> = vec![Vec::new(); layers.len() + 1];
  let mut tmp: Vec<i8x16> = Vec::new();
  let goal_uc = unicount(goal);

  for i in 0..layers.len() {
    let output = layers[i].layer;

    for j in 0..layers.len() {
      let output2 = i8x16::shuffle(&layers[j].layer, &output);

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