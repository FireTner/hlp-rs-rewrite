use std::time::Instant;

use crate::unicount::unicount;
use crate::pairs::gen_pairs;
use crate::layer::Layer;
use crate::lut::gen_lut;
use crate::vec::i8x16;
use crate::dfs::{first_layer, ITERATIONS};
use crate::tables::Tables;
use crate::eqmask::gen_eqmask;

const MAX_DEPTH: usize = 42;

pub fn search(goal: [i8; 16]) {
  let goal_vec: i8x16 = i8x16::from_array(&goal);

  // Count the number of unique elements target has
  let goal_uc = unicount(goal_vec);
  println!("Unique number of elements for target: {}", goal_uc);

  // Pregenerate look up table
  let layers: Vec<Layer> = gen_lut(goal_vec);
  println!("Size of look up table: {}", layers.len());

  // Pregenerate pairs
  let pairs: (Vec<Vec<usize>>, usize) = gen_pairs(&layers, goal_vec);
  println!("Amount of pairs: {}", pairs.1);

  // Generate equality mask for goal
  let goal_eqmask: [i32; 15] = gen_eqmask(goal_vec);

  // Search
  let start_time = Instant::now();
  for currentlayer in 1..=MAX_DEPTH {
    println!("Starting search at {}", currentlayer);
    if first_layer(&Tables { cur_layer: currentlayer, layers: layers.clone(), pairs: pairs.0.clone(), goal: goal_vec, goal_eqmask: goal_eqmask }) {
      println!("Found at {} depth", currentlayer);
      return;
    }
    println!("Iterations: {}", unsafe { ITERATIONS });


    println!("Finished searching in {} ms\n", start_time.elapsed().as_millis());
  }
}