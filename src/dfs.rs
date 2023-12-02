// use crate::layer::Layer;
use crate::vec::i8x16;
use crate::tables::Tables;
use crate::applylayer::apply_layer;

pub fn dfs(depth: &usize, input: &i8x16, prev_index: &usize, tables: &Tables) -> bool {
  if *depth == tables.cur_layer { return last_layer(&input, &prev_index, &tables); }

  for i in 0..tables.pairs[*prev_index].len() {
    let index = tables.pairs[*prev_index][i];
    let output = apply_layer(&input, &tables.layers[index].layer, &tables.goal);

    if dfs(&(depth + 1), &output, &index, &tables) { return true; }
  }

  false
}

pub fn last_layer(input: &i8x16, prev_index: &usize, tables: &Tables) -> bool {
  for i in 0..tables.pairs[*prev_index].len() {
    let index = tables.pairs[*prev_index][i];
    let output = i8x16::shuffle(&tables.layers[index].layer, &input);

    if output == tables.goal { return true; }
  }

  return false;
}

pub fn first_layer(tables: &Tables) -> bool {
  if tables.cur_layer == 0 { return false; }

  for i in 0..tables.layers.len() {
    if dfs(&1, &tables.layers[i].layer, &i, &tables) { return true; }
  }

  return false;
}