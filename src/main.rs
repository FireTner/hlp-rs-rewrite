mod vec;
mod lut;
mod dfs;
mod layer;
mod pairs;
mod cache;
mod search;
mod tables;
mod eqmask;
mod unicount;
mod distcheck;
mod applylayer;

extern crate num;
#[macro_use]
extern crate num_derive;

use crate::search::search;

// 13,13,13,12,11,10, 9, 8, 7, 6, 5, 4, 3, 2, 1,15 -> 2 deep
// 10,10,10,10,10,10,10,10, 5, 5, 5, 5, 5, 5, 6, 7 -> 3 deep
//  5, 5, 4, 2, 2, 2, 2, 2, 8, 9,10,11,12,13,14,15 -> 4 deep
//  3, 3, 2, 2,12,13,11,10, 9, 8, 3, 2, 1, 0, 0, 1 -> 5 deep
//  3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3 -> pi

fn main() {
  if !is_x86_feature_detected!("avx") || !is_x86_feature_detected!("sse3") || !is_x86_feature_detected!("sse4.1") || !is_x86_feature_detected!("sse4.2") {
    eprintln!("Missing some features");
    return;
  }

  let goal: [i8; 16] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3];

  search(goal);
}
