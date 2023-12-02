use crate::vec::i8x16;
use crate::cache::*;
use crate::tables::Tables;
use crate::applylayer::apply_layer;

pub static mut ITERATIONS: usize = 0;

// Depth first search function
// Takes depth, output of previous function, previous index, and `tables` as parameters
// Returns true if it found the correct solution otherwise false
//
// This function either calls itself recursively until it depth is same as tables.current_layer
// then it calls last_layer to check if it found a solution
pub fn dfs(depth: &i8, input: &i8x16, prev_index: &usize, tables: &Tables, cache: &mut Box<Cache>) -> bool {
  if cache.contains(&input, &depth) { return false; }
  if *depth == tables.cur_layer { return last_layer(&input, &prev_index, &tables); }

  let pair = &tables.pairs[*prev_index];

  for i in 0..pair.len() {
    let index = pair[i];
    let layer = &tables.layers[index].layer;
    let output = apply_layer(&input, layer, &tables);
    
    if output == i8x16::zero() { continue; }

    unsafe { ITERATIONS += 1; }

    if dfs(&(depth + 1), &output, &index, &tables, cache) { return true; }
  }

  return false;
}

// Last layer search function
// Takes output of previous function, previous index, and `tables` as parameters
// Returns true if it found the correct solution otherwise false
//
// Loops over possible layers and returns true if solution was found
#[inline(always)]
pub fn last_layer(input: &i8x16, prev_index: &usize, tables: &Tables) -> bool {
  let pair = &tables.pairs[*prev_index];

  for i in 0..pair.len() {
    let index = pair[i];
    let layer = tables.layers[index].layer;
    let output = i8x16::shuffle(&layer, &input);

    unsafe { ITERATIONS += 1 };

    if output == tables.goal { return true; }
  }

  return false;
}

// First layer search function
// Takes `tables` as parameters
// Returns true if found a solution otherwise false
//
// Calls dfs and recursively tries to bruteforce a solution
#[inline(always)]
pub fn first_layer(tables: &Tables) -> bool {
  if tables.cur_layer <= 2 { return false; }
  unsafe {
    ITERATIONS = 0;
    HIT = 0;
    MISS = 0;
    NEW = 0;  
  };
  let mut cache: Box<Cache> = Box::new(Cache::new());

  for i in 0..tables.layers.len() {
    let output = apply_layer(&i8x16::start(), &tables.layers[i].layer, &tables);
    
    if output == i8x16::zero() { continue; }

    if dfs(&2, &output, &i, &tables, &mut cache) { return true; }
  }

  return false;
}