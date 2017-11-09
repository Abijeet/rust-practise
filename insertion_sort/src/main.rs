fn main() {
    let mut arr: [i32; 7] = [4, 22, 11, 56, 13, 19, 24];

    print_array_with_msg("Before sorting: ".to_string(), &arr);

    for i in 1 .. arr.len() {
      let elem = arr[i];
      sort_from_right(&mut arr, i, elem);
    }
    print_array_with_msg("After sorting: ".to_string(), &arr);
}

fn sort_from_right (arr: &mut [i32], right_index: usize, value: i32) -> &mut [i32] {
  // println!("{}, {}", right_index, value);
  let mut i = right_index - 1;
  while i != 0 && arr[i] > value {
    // move elements over
    arr[i + 1] = arr[i];
    arr[i] = value;
    i -= 1;
  }
  return arr;
}

fn print_array_with_msg (msg_to_print: String, arr: &[i32]) {
   if arr.len() == 0 {
     return;
   }
   print!("{}", msg_to_print);
   let mut arr_elements: String = String::from("");
   for element in arr.iter() {
     arr_elements.push_str(" ");
     arr_elements.push_str(element.to_string().as_ref());
   }

   println!("{}", arr_elements.trim());
}