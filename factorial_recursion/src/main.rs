use std::io;

fn main() {
    println!("Enter a positive number to calculate factorial: ");
    let mut number:u64;
    let min_val = 1;
    let max_val = 20;
    loop {
      let mut str_number = String::new();
      io::stdin().read_line(&mut str_number).expect("Failed to read the number");

      number = match str_number.trim().parse() {
          Ok(num) => num,
          Err(_) => continue
      };

      if number < min_val || number > max_val {
        println!("Please enter a number between {} and {}.", min_val, max_val);
        continue;
      }

      break;
    }
    let factorial_val = factorial(number);

    println!("Factorial of number {} is {}", number, factorial_val);
}

fn factorial(number: u64) -> u64 {
  if number == 0 {
    return 1;
  }

  return number * factorial(number - 1);
}