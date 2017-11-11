fn main() {
    let mut arr: [i32; 8] = [7, 2, 5, 3, 11, 23, 12, 4];

    println!("Before: {:?}", arr);

    for i in 1 .. arr.len() {
      let elem = arr[i];
      sort_from_right(&mut arr[0 .. i + 1], elem);
    }
    println!("After: {:?}", arr);
}

fn sort_from_right (arr: &mut [i32], value: i32) -> &mut [i32] {
  if arr.len() <= 1 {
    return arr;
  }

  let mut i = arr.len() - 1;
  println!("Step {}: {:?}",i, arr);
  while i > 0 && arr[i] < arr[i - 1] {
    // move elements over
    arr[i] = arr[i - 1];
    arr[i - 1] = value;
    i -= 1;
  }
  return arr;
}