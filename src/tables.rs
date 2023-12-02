use crate::layer::Layer;
use crate::vec::i8x16;

pub struct Tables {
  pub cur_layer: usize,
  pub layers: Vec<Layer>,
  pub pairs: Vec<Vec<usize>>,
  pub goal: i8x16,
  pub goal_eqmask: [i32; 15],
}