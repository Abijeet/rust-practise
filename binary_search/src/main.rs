use std::io;

fn main() {
  // array of prime numbers, un-sorted
  let mut arr = [2, 3, 5, 7, 11, 13, 17, 73, 23, 29, 31, 37, 41,
    43, 47, 53, 59, 61, 67, 71, 19, 79, 149, 89, 97, 101, 103,
    107, 109, 113, 127, 131, 137, 139, 83, 151, 157, 163,
    167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 231];

  // sort the array.
  asc_sort(&mut arr);

  // get input from the user
  println!("Please enter a number: ");
  let mut search = String::new();
  io::stdin().read_line(&mut search).expect("Failed to read line.");
  let num_search:i32 = search.trim().parse().expect("Please enter a number!");

  // find the element in the array
  let position = get_index(&arr, num_search);
  if position == -1 {
    println!("The element was not found!");
  } else {
    println!("The element {} was found at position #{}.", num_search, position + 1);
  }
}

// Sort the array
fn asc_sort(arr_search: &mut [i32]) {
  let len = arr_search.len();
  let mut swap = false;
  let mut curr_index = 0;
  let mut tmp;
  loop {
    if curr_index + 1 == len {
      curr_index = 0;
      if !swap {
        break;
      }
      swap = false;
    }

    tmp = arr_search[curr_index];
    if arr_search[curr_index] > arr_search[curr_index + 1] {
      arr_search[curr_index] = arr_search[curr_index + 1];
      arr_search[curr_index + 1] = tmp;
      swap = true;
    }

    curr_index += 1;
  }
}

// Return the index of the element to be returned, else return -1.
// Implemented using binary search.
fn get_index(arr_search: &[i32], num_search:i32) -> i32 {
  let mut index:i32 = -1;
  let mut max = arr_search.len() - 1;
  let mut min = 0;

  loop {
    let avg = (max  + min) / 2;

    let curr_val = arr_search[avg];
    if curr_val == num_search {
      index = avg as i32;
      break;
    }

    if avg == min {
      break;
    }

    if num_search > curr_val {
      // search value is greater than the current value.
      // value to be looked for is on the right
      min = avg + 1;
    } else {
      // search value is less than the current value.
      // value to be looked for is on the left
      max = avg - 1;
    }
  }

  index
}