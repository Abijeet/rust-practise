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
    merge_sort(numbers.as_mut_slice(), 0, size);

    println!("The numbers sorted are: {:?}", numbers);
}

fn merge(numbers: &mut [i32], first: usize, mid:usize, end:usize) -> &mut [i32] {
  let mut first_half: Vec<i32> = vec![];
  let mut second_half: Vec<i32>  = vec![];

  for i in first..mid + 1{
    first_half.push(numbers[i]);
  }

  for i in mid + 1 .. end + 1{
    second_half.push(numbers[i]);
  }

  let mut i = 0;
  let mut j = 0;
  let mut k = first;

  while i < first_half.len() && j < second_half.len() {
    if first_half[i] < second_half[j] {
      numbers[k]  = first_half[i];
      i += 1;
    } else {
      numbers[k]  = second_half[j];
      j += 1;
    }
    k += 1;
  }

  while i <  first_half.len() {
    numbers[k] = first_half[i];
    k += 1;
    i += 1;
  }

  while j < second_half.len() {
    numbers[k] = second_half[j];
    k += 1;
    j += 1;
  }

  return numbers;
}

fn merge_sort(numbers: &mut [i32], start: usize, end: usize) -> &mut [i32] {
  if start < end {
    let mid = (start + end) / 2;
    merge_sort(numbers, start, mid);
    merge_sort(numbers, mid + 1, end);
    merge(numbers, start, mid, end);

  }
  return numbers;
}