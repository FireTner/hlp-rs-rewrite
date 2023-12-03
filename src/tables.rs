use crate::distcheck::DistanceTable;
use crate::layer::Layer;
use crate::vec::i8x16;

#[derive(Clone)]
pub struct Tables {
  pub cur_layer: i8,
  pub layers: Vec<Layer>,
  pub pairs: Vec<Vec<usize>>,
  pub goal: i8x16,
  pub goal_eqmask: [i32; 15],
  pub dist_table: DistanceTable,
}