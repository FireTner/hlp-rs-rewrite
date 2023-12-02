use crate::vec::i8x16;

const CACHE_SIZE: u32 = 1 << 25;

pub struct Cache {
  pub depth: [i8; CACHE_SIZE as usize],
  pub layer: [i8x16; CACHE_SIZE as usize],
}

pub static mut HIT: usize = 0;
pub static mut NEW: usize = 0;
pub static mut MISS: usize = 0;

impl Cache {
  // Creates an empty cache
  pub fn new() -> Self {
    Cache {
      depth: [0; CACHE_SIZE as usize],
      layer: [i8x16::zero(); CACHE_SIZE as usize],
    }
  }

  // Checks if cache contains a value at valid depth
  // Takes value and depth as parameters
  // Returns boolean if value is found at a valid depth
  //
  // If a value is found at lower or same depth return true otherwise false
  #[inline(always)]
  pub fn contains(&mut self, value: &i8x16, depth: &i8) -> bool {
    let pos: u32 = value.hash() & (CACHE_SIZE - 1);
    
    let cached_depth = self.depth[pos as usize];
    let cached_value = self.layer[pos as usize];

    let found = cached_depth <= *depth && cached_value == *value;
    
    unsafe {
      if found { HIT += 1; }
      else if self.layer[pos as usize] == i8x16::zero() { NEW += 1; }
      else { MISS += 1; }
    }

    if found { return true; }

    self.depth[pos as usize] = *depth;
    self.layer[pos as usize] = *value;
    return false;
  }
}