use crate::applylayer::apply_layer;

use crate::tables::Tables;
use crate::layer::Layer;
use crate::vec::i8x16;
use crate::{cache::*, distcheck};

const MAX_DEPTH: usize = 42;

pub struct Search {
  pub iterations: usize,
  pub result: [Layer; MAX_DEPTH as usize],
  pub tables: Tables,
  pub cache: Box<Cache>,
  pub found: bool,
}

// Depth first search function
// Takes depth, output of previous function, previous index, and `tables` as parameters
// Returns true if it found the correct solution otherwise false
//
// This function either calls itself recursively until it depth is same as tables.current_layer
// then it calls last_layer to check if it found a solution
impl Search {
  pub fn dfs(&mut self, depth: &i8, input: &i8x16, prev_index: &usize) -> bool {
    if self.cache.contains(&input, &depth) { return false; }
    if *depth == self.tables.cur_layer { return self.last_layer(&input, &prev_index); }

    let paircount = self.tables.pairs[*prev_index].len();
    for i in 0..paircount {
      let index = self.tables.pairs[*prev_index][i];
      let layer = self.tables.layers[index];
      let output = apply_layer(&input, &layer.layer, &mut self.tables);
      
      if output == i8x16::zero() { continue; }
      if self.tables.dist_table.distnace(&output.as_array(), self.tables.cur_layer - depth) { continue; }

      self.iterations += 1;

      self.dfs(&(depth + 1), &output, &index);
      if self.found {
        self.result[(depth - 1) as usize] = layer;
        return true;
      }
    }

    return false;
  }

  // Last layer search function
  // Takes output of previous function, previous index, and `tables` as parameters
  // Returns true if it found the correct solution otherwise false
  //
  // Loops over possible layers and returns true if solution was found
  #[inline(always)]
  pub fn last_layer(&mut self, input: &i8x16, prev_index: &usize) -> bool {
    let pair = &self.tables.pairs[*prev_index];

    for i in 0..pair.len() {
      let index = pair[i];
      let layer = self.tables.layers[index];
      let output = i8x16::shuffle(&layer.layer, &input);

      self.iterations += 1;

      if output == self.tables.goal {
        self.result[(self.tables.cur_layer - 1) as usize] = layer;
        return true;
      }
    }

    return false;
  }

  // First layer search function
  // Takes `tables` as parameters
  // Returns true if found a solution otherwise false
  //
  // Calls dfs and recursively tries to bruteforce a solution
  #[inline(always)]
  pub fn first_layer(tables: &mut Tables) -> Self {
    let mut result = Search {
      iterations: 0,
      result: [Layer::empty(); MAX_DEPTH],
      tables: tables.clone(),
      cache: Box::new(Cache::new()),
      found: false,
    };

    if tables.cur_layer <= 2 { return result; }

    for i in 0..tables.layers.len() {
      let layer = tables.layers[i];
      let output = apply_layer(&i8x16::start(), &layer.layer, tables);
      
      if output == i8x16::zero() { continue; }

      result.dfs(&2, &output, &i);

      if result.found {
        break;
      }
    }

    return result;
  }
}