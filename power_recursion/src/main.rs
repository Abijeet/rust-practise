use std::io;

fn main() {
    let x = get_number_input("Enter the number:", -100.00, 100.00);
    let n = get_number_input("Enter the exponent: ", -20.00, 20.00);
    let result = power(x, n);
    println!("{} raised to the power of {} is {:.2}.", x, n, result);
}

fn get_number_input (prompt: &str, min_val: f64, max_val: f64) -> f64 {
  let mut number:f64;
  println!("{}", prompt);
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
    return number;
}
fn power (x:f64, n: f64) -> f64 {
  if n == 0.0 {
    return 1.0;
  }

  if n < 0.0 {
    // less than 0
    return 1.0 / power(x, (n * -1.0));
  } else if n % 2.0 == 0.0 {
    // even
    let result = power(x, n / 2.0);
    return result * result;
  } else {
    // odd
    return x * power(x, n - 1.0);
  }
}