use std::io;

fn main() {
    let mut numbers = String::new();
    println!("Enter numbers to sort, separated by space -", );
    io::stdin().read_line(&mut numbers).ok().expect("read error");

    let mut numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let size = numbers.len() - 1;
    quick_sort(numbers.as_mut_slice(), 0,  size);

    println!("The numbers sorted are: {:?}", numbers);
}

fn quick_sort (numbers: &mut [i32], first_index: usize, last_index: usize) {
  if numbers.len() == 0 {
    return;
  }

  if first_index < last_index {
    let q = partition(numbers, first_index, last_index);
    println!("{}", q);
    if q > 0 {
      quick_sort(numbers, first_index, q - 1);
    }

    quick_sort(numbers, q + 1, last_index);
  }
}

fn partition(numbers: &mut [i32], first_index: usize, last_index: usize) -> usize {
  // taking the last element as the pivot.
  let pivot_index = last_index;
  let pivot_value = numbers[pivot_index];

  let mut p = first_index;
  let mut j = first_index;
  while j < pivot_index  {
    if numbers[j] < pivot_value {
      swap(numbers, j, p);
      p += 1;
    }
    j += 1;
  }
  swap(numbers, p, pivot_index);
  return p;
}

fn swap (numbers: &mut [i32], index: usize, index2: usize) {
  let tmp = numbers[index];
  numbers[index] = numbers[index2];
  numbers[index2] = tmp;
}
