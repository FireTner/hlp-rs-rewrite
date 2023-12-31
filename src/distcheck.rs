use crate::vec::i8x16;
use crate::layer::Layer;

// number of elements
const ELEMENT_COUNT: usize = 4;
// number of tables
const TABLE_COUNT: usize = 4;
// maximum depth for precalculating tables
const MAX_TABLE_DEPTH: i8 = 10;

// Don't change
const TABLE_SIZE: usize = 1 << (ELEMENT_COUNT * 4);
// Offsets for each element
const OFFSET_TABLE: [usize; TABLE_COUNT] = [0, 4, 8, 12];

#[derive(Clone)]
pub struct DistanceTable {
  tables: [[i8; TABLE_SIZE]; TABLE_COUNT],
  totals: [usize; TABLE_COUNT],
  total: usize,
}

impl DistanceTable {
  // Generates a table of depths
  // Takes depth and look up table as parameters
  //
  // Generates a table of depths that can reach the next depth
  // from the current depth supplied by depth parameter
  //
  // This works by trying every input, applying layer on it
  // and checking if it reaches already recorded output
  // then it stores the depth of output + 1
  pub fn generate(&mut self, depth: &i8, layers: &Vec<Layer>) {
    for index in 0..layers.len() {
      for input in 0..TABLE_SIZE {
        let mut input_array: [i8; 16] = [0; 16];
        let layer = layers[index].layer;

        for i in 0..ELEMENT_COUNT {
          input_array[i] = (input >> (i * 4)) as i8 & 0xF;
        }

        let input_vector = i8x16::from_array(&input_array);
        let output_array = layer.shuffle(&input_vector).as_array();
        let output = Self::to_offset(&output_array, 0);

        for i in 0..TABLE_COUNT {
          let table = &self.tables[i];

          if table[output] == *depth && table[input] == 100 {
            self.tables[i][input] = depth + 1;
            self.totals[i] += 1;
            self.total += 1;
          }
        }
      }
    }
  }

  // Makes a new distance table
  // Takes goal and look up table as parameters
  // Returns DistanceTable
  //
  // Sets a goal and repeatedly calls generate until
  // there's no new depths
  pub fn new(goal: &i8x16, tables: &Vec<Layer>) -> DistanceTable {
    let mut result = DistanceTable {
      tables: [[100; TABLE_SIZE]; TABLE_COUNT],
      totals: [0; TABLE_COUNT],
      total: 0,
    };

    // Sets goal as depth 0
    let goal_array = goal.as_array();
    for i in 0..TABLE_COUNT {
      let offset = Self::to_offset(&goal_array, OFFSET_TABLE[i]);
      result.tables[i][offset] = 0;
    }
    
    // Tries every depth until it can't generate more
    let mut prev_total = 0;
    for i in 0..MAX_TABLE_DEPTH {
      result.generate(&i, &tables);

      if prev_total == result.total { break; }
      prev_total = result.total;

      for i in 0..TABLE_COUNT {
        print!("\t{}: {} ", i, result.totals[i]);
      }
      println!("\t({})", result.total);
    }

    result
  }

  #[inline(always)]
  fn to_offset(value: &[i8; 16], start: usize) -> usize {
    let mut result = 0;

    for i in 0..ELEMENT_COUNT {
      result |= (value[(start + i) & 0xF] as usize) << (i * 4);
    }

    return result;
  }

  #[inline(always)]
  pub fn distnace(&self, value: &[i8; 16], threshold: i8) -> bool {
    for i in 0..TABLE_COUNT {
      let offset = Self::to_offset(value, OFFSET_TABLE[i]);
      if self.tables[i][offset] > threshold {
        return true;
      }
    }
    return false;
  }
}