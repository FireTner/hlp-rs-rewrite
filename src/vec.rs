use std::ops::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct i8x16 {
  value: __m128i,
}

// Implementation of declaration
impl i8x16 {
  // Create i8x16 from an array
  pub fn from_array(value: &[i8; 16]) -> Self {
    unsafe {
      i8x16 { 
        value: _mm_set_epi8(value[15], value[14], value[13], value[12], value[11], value[10], value[9], value[8], value[7], value[6], value[5], value[4], value[3], value[2], value[1], value[0]),
      }
    }
  }

  // Create i8x16 from a simd vector
  pub fn from_vec(value: __m128i) -> Self {
    i8x16 { value: value }
  }

  // Create i8x16 with all elements being set to `value`
  pub fn from_imm(value: i8) -> Self {
    unsafe { i8x16 { value: _mm_set1_epi8(value) } }
  }

  // Create i8x16 with all elements being set to 0
  pub fn zero() -> Self { i8x16::from_imm(0) }

  // Create i8x16 with elements starting from 0 to 15
  pub fn start() -> Self { i8x16::from_array(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]) }

  // Returns an array from i8x16
  pub fn as_array(&self) -> [i8; 16] {
    let mut tmp: [i8; 16] = [0; 16];

    unsafe { _mm_storeu_si128(tmp.as_mut_ptr() as *mut __m128i, self.value) }

    return tmp;
  }

  // Compares the two values and returns the bigger
  pub fn max(&self, other: &Self) -> Self {
    unsafe { i8x16::from_vec(_mm_max_epi8(self.value, other.value)) }
  }

  // Computes not with `a` and returns bitwise and with `b`
  pub fn andnot(&self, other: &Self) -> Self {
    unsafe { i8x16::from_vec(_mm_andnot_si128(self.value, other.value)) }
  }

  // Compares every element and sets to -1 if true otherwise sets to 0 
  pub fn cmplt(&self, other: &Self) -> Self {
    unsafe { i8x16::from_vec(_mm_cmplt_epi8(self.value, other.value)) }
  }

  // Does dst[i] = a[b[i]]
  // For layers use b as input and a as mask
  pub fn shuffle(&self, other: &i8x16) -> Self {
    unsafe { i8x16::from_vec(_mm_shuffle_epi8(self.value, other.value)) }
  }
}

// Implementation of addition
impl Add for i8x16 {
  type Output = i8x16;

  fn add(self, other: Self) -> i8x16 {
    unsafe { i8x16 { value: _mm_add_epi8(self.value, other.value), } }
  }
}

// Implementation of subtraction
impl Sub for i8x16 {
  type Output = i8x16;

  fn sub(self, other: Self) -> i8x16 {
    unsafe { i8x16 { value: _mm_sub_epi8(self.value, other.value), } }
  }
}

// Implementation of xor
impl BitXor for i8x16 {
  type Output = i8x16;

  fn bitxor(self, other: Self) -> i8x16 {
    unsafe { i8x16 { value: _mm_xor_si128(self.value, other.value), } }
  }
}

// Implementation of equality check
impl PartialEq for i8x16 {
  fn eq(&self, other: &i8x16) -> bool {
    unsafe {
      let tmp = (*self ^ *other).value;
      _mm_testz_si128(tmp, tmp) != 0
    }
  }

  fn ne(&self, other: &i8x16) -> bool {
    unsafe {
      let tmp = (*self ^ *other).value;
      _mm_testz_si128(tmp, tmp) == 0
    }
  }
}