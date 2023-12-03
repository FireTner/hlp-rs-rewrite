use std::time::Instant;
use chrono::Local;

use crate::unicount::unicount;
use crate::pairs::gen_pairs;
use crate::layer::Layer;
use crate::lut::gen_lut;
use crate::vec::i8x16;
use crate::dfs::Search;
use crate::tables::Tables;
use crate::eqmask::gen_eqmask;
use crate::distcheck::DistanceTable;

pub const MAX_DEPTH: i8 = 42;

  // SEARCH
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

    // Generate a distance table
    let dist_table = DistanceTable::new(&goal_vec, &layers);

    // Make a table that will be used for the search
    let table = &mut Tables {
      cur_layer: 0,
      layers: layers,
      pairs: pairs.0,
      goal: goal_vec,
      goal_eqmask: goal_eqmask,
      dist_table: dist_table,
    };

    // Search
    let start_time = Instant::now();
    for currentlayer in 1..=MAX_DEPTH {
      table.cur_layer = currentlayer;
      let local_time = Local::now();
      
      println!("[{}] Starting search at {}", local_time.format("%H:%M:%S%.f"), currentlayer);
  
      let result = Search::first_layer(table);

      if result.found {
        println!("Found at {} depth", currentlayer);
        for i in 0..currentlayer {
          result.result[i as usize].print();
        }
        println!();
        return;
      }
  
      println!("\tIterations: {}", result.iterations );
      println!("\tHits: {}", result.cache.hit);
      println!("\tMisses: {}", result.cache.miss);
      println!("\tInitialized: {}", result.cache.new);
  
      println!("\tFinished searching in {} ms\n", start_time.elapsed().as_millis());
    }
  }