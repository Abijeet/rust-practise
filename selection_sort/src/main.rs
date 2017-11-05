fn main(){
  let mut arr = [
    1, 2, 4, 3, 6, 5
  ];

  print_array_with_msg("Before sorting: ".to_string(), &arr);

  let mut total_loops: usize = 0;
  let array_length = arr.len();
  loop {
    if total_loops == (array_length - 1) {
      break;
    }
    let min_index = get_min_index(&arr, total_loops);
    if min_index == total_loops {
      // nothing swapped
      total_loops += 1;
      continue;
    }

    // swap the elements
    let temp = arr[total_loops];
    arr[total_loops] = arr[min_index];
    arr[min_index] = temp;
    total_loops += 1;
  }
  print_array_with_msg("After sorting: ".to_string(), &arr);
}

fn get_min_index(arr: &[i32], start_index: usize) -> usize {
  let array_lenth = arr.len();
  if array_lenth == 0 || array_lenth < start_index {
    panic!("Invalid inputs specified for function. Array Length : {}, Start index: {}", array_lenth, start_index);
  }
  let mut smallest_index: usize = start_index;
  let mut current_index: usize = start_index + 1;
  loop {
    if current_index == array_lenth {
      break;
    }

    if arr[smallest_index] > arr[current_index] {
      smallest_index = current_index;
    }

    current_index += 1;
  }

  return smallest_index;
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