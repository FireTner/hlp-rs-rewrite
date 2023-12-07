#[inline(always)]
pub fn distance(goal: &[i8; 16], value: &[i8; 16], threshold: i8) -> bool {
  let mut arr: [i32; 16] = [0; 16];
  for i in 0..16 { //zip(start, goal)
    arr[i] = (value[i] as i32) << 4;
    arr[i] |= goal[i] as i32;
  }
  arr.sort();
  let mut total: i8 = 0;
  for i in 0..15 {//"big jump" magic
    let a1 = arr[i] & 15;
    let b1 = arr[i + 1] & 15;
    let diff1 = i32::abs(a1 - b1);
    
    let a2 = (arr[i] >> 4) & 15;
    let b2 = (arr[i + 1] >> 4) & 15;
    let diff2 = i32::abs(a2 - b2);

    if diff1 > diff2 { total += 1; }
  }

  return total > threshold; //if distance is too far return 1 to prune it
}