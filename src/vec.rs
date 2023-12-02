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
  #[inline(always)]
  pub fn from_array(value: &[i8; 16]) -> Self {
    unsafe {
      i8x16 { 
        value: _mm_set_epi8(value[15], value[14], value[13], value[12], value[11], value[10], value[9], value[8], value[7], value[6], value[5], value[4], value[3], value[2], value[1], value[0]),
      }
    }
  }

  // Create i8x16 from a simd vector
  #[inline(always)]
  pub fn from_vec(value: __m128i) -> Self {
    i8x16 { value: value }
  }

  // Create i8x16 with all elements being set to `value`
  #[inline(always)]
  pub fn from_imm(value: i8) -> Self {
    unsafe { i8x16 { value: _mm_set1_epi8(value) } }
  }

  // Create i8x16 with all elements being set to 0
  #[inline(always)]
  pub fn zero() -> Self { i8x16::from_imm(0) }

  // Create i8x16 with elements starting from 0 to 15
  #[inline(always)]
  pub fn start() -> Self { i8x16::from_array(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]) }

  // Returns an array from i8x16
  #[inline(always)]
  pub fn as_array(&self) -> [i8; 16] {
    let mut tmp = [0; 16];

    unsafe { _mm_storeu_si128(tmp.as_mut_ptr() as *mut __m128i, self.value) }

    return tmp;
  }

  // Returns an array from i8x16
  #[inline(always)]
  pub fn as_big_array(&self) -> [u32; 4] {
    let mut tmp = [0; 4];

    unsafe { _mm_storeu_si128(tmp.as_mut_ptr() as *mut __m128i, self.value) }

    return tmp;
  }

  // Compares the two values and returns the bigger
  #[inline(always)]
  pub fn max(&self, other: &Self) -> Self {
    unsafe { i8x16::from_vec(_mm_max_epi8(self.value, other.value)) }
  }

  // Computes not with `a` and returns bitwise and with `b`
  #[inline(always)]
  pub fn andnot(&self, other: &Self) -> Self {
    unsafe { i8x16::from_vec(_mm_andnot_si128(self.value, other.value)) }
  }

  // Compares every element and sets to -1 if true otherwise sets to 0
  #[inline(always)]
  pub fn cmplt(&self, other: &Self) -> Self {
    unsafe { i8x16::from_vec(_mm_cmplt_epi8(self.value, other.value)) }
  }

  // Does dst[i] = a[b[i]]
  // For layers use b as input and a as mask
  #[inline(always)]
  pub fn shuffle(&self, other: &i8x16) -> Self {
    unsafe { i8x16::from_vec(_mm_shuffle_epi8(self.value, other.value)) }
  }

  // Make equality mask
  // Create a bitmap for every element if equal then 1 otherwise 0
  #[inline(always)]
  pub fn eqmask(&self, other: &i8x16) -> i32 {
    unsafe { _mm_movemask_epi8(_mm_cmpeq_epi8(self.value, other.value)) }
  }

  // Hash function for i8x16
  // Uses crc32 intrinstinc to hash i8x16 to 32 bit unsigned integer
  #[inline(always)]
  pub fn hash(&self) -> u32 {
    let tmp = self.as_big_array();
    let mut result = 0;

    unsafe {
      result = _mm_crc32_u32(result, tmp[0]);
      result = _mm_crc32_u32(result, tmp[1]);
      result = _mm_crc32_u32(result, tmp[2]);
      result = _mm_crc32_u32(result, tmp[3]);
    }

    return result;
  }
}

// Implementation of addition
impl Add for i8x16 {
  type Output = i8x16;

  #[inline(always)]
  fn add(self, other: Self) -> i8x16 {
    unsafe { i8x16 { value: _mm_add_epi8(self.value, other.value), } }
  }
}

// Implementation of subtraction
impl Sub for i8x16 {
  type Output = i8x16;

  #[inline(always)]
  fn sub(self, other: Self) -> i8x16 {
    unsafe { i8x16 { value: _mm_sub_epi8(self.value, other.value), } }
  }
}

// Implementation of xor
impl BitXor for i8x16 {
  type Output = i8x16;

  #[inline(always)]
  fn bitxor(self, other: Self) -> i8x16 {
    unsafe { i8x16 { value: _mm_xor_si128(self.value, other.value), } }
  }
}

// Implementation of equality check
impl PartialEq for i8x16 {
  #[inline(always)]
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