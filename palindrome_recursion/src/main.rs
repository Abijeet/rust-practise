use std::io;

fn main() {
  let user_input = get_string_input("Enter a string to check for palindrome", 25, 1);
  let mut tmp_user_input = user_input.clone();
  let is_palindrome = is_palindrome(&mut tmp_user_input);
  if is_palindrome {
    println!("{} is a palindrome.", user_input);
  } else {
    println!("{} is NOT a palindrome.", user_input);
  }
}

fn get_string_input (prompt: &str, max_value: usize, min_value: usize) -> String {
  println!("{}", prompt);
  let mut input = String::new();
  loop {
      io::stdin().read_line(&mut input).expect("Failed to read the number");
      input = input.trim().to_string();
      let input_len = input.len();
      if input_len < min_value || input_len > max_value {
        println!("Please enter a string between length - {} and {}.", min_value, max_value);
        continue;
      }

      break;
    }

    return input;
}

fn last_character (word: &str) -> char {
  if word.is_empty() {
    panic!("last_character called with empty string.");
  }
  let ch = word.chars().nth(word.len() - 1).unwrap();
  return ch;
}

fn first_character (word: &str) -> char {
  if word.is_empty() {
    panic!("first_character called with empty string.");
  }
  let ch = word.chars().nth(0).unwrap();
  return ch;
}

fn middle_characters (word: &str) -> &str {
  let len = word.len();
  if len <= 1 {
    panic!("middle_characters called with a word less than length 2.");
  }

  let middle_word = &word[1..len - 1];
  return middle_word;
}

fn is_palindrome (word: &str) -> bool {
  if word.len() <= 1 {
    return true;
  }

  if first_character(word) == last_character(word) {
    return is_palindrome(middle_characters(&word));
  }
  return false;
}