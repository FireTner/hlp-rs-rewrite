use crate::vec::i8x16;
use crate::tables::Tables;
use crate::applylayer::apply_layer;

// Depth first search function
// Takes depth, output of previous function, previous index, and `tables` as parameters
// Returns true if it found the correct solution otherwise false
//
// This function either calls itself recursively until it depth is same as tables.current_layer
// then it calls last_layer to check if it found a solution
pub fn dfs(depth: &usize, input: &i8x16, prev_index: &usize, tables: &Tables) -> bool {
  if *depth == tables.cur_layer { return last_layer(&input, &prev_index, &tables); }

  for i in 0..tables.pairs[*prev_index].len() {
    let index = tables.pairs[*prev_index][i];
    let output = apply_layer(&input, &tables.layers[index].layer, &tables.goal);

    if dfs(&(depth + 1), &output, &index, &tables) { return true; }
  }

  false
}

// Last layer search function
// Takes output of previous function, previous index, and `tables` as parameters
// Returns true if it found the correct solution otherwise false
//
// Loops over possible layers and returns true if solution was found
pub fn last_layer(input: &i8x16, prev_index: &usize, tables: &Tables) -> bool {
  for i in 0..tables.pairs[*prev_index].len() {
    let index = tables.pairs[*prev_index][i];
    let output = i8x16::shuffle(&tables.layers[index].layer, &input);

    if output == tables.goal { return true; }
  }

  return false;
}

// First layer search function
// Takes `tables` as parameters
// Returns true if found a solution otherwise false
//
// Calls dfs and recursively tries to bruteforce a solution
pub fn first_layer(tables: &Tables) -> bool {
  if tables.cur_layer == 0 { return false; }

  for i in 0..tables.layers.len() {
    if dfs(&1, &tables.layers[i].layer, &i, &tables) { return true; }
  }

  return false;
}