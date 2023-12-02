use crate::vec::i8x16;
use crate::unicount::unicount;
use crate::layer::*;

pub fn gen_lut(goal: i8x16) -> Vec<Layer> {
  let start: i8x16 = i8x16::start();
  let mut layers: Vec<Layer> = Vec::new();
  let goal_uc = unicount(goal);

  for configuration in 0..CONFCOUNT {
    let output = Layer::generate(start, configuration);

    if output.layer == start { continue; }
    if output.layer == i8x16::zero() { continue; }
    if unicount(output.layer) < goal_uc { continue; }
    if layers.contains(&output) { continue; }

    layers.push(output);
  }

  return layers;
}