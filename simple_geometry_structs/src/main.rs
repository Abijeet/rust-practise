use std::f32;

fn main() {
  let rect_height = 12;
  let rect_width = 20;
  let square_size = 10;
  let circle_radius = 15.0;
  let triangle_sides = (10.0, 6.0, 7.0);

  let rect = Rectangle { width: rect_width, height: rect_height };
  let sq = Rectangle::square(square_size);
  let circle = Circle { radius: circle_radius};
  let triangle = Triangle { side1: triangle_sides.0, side2: triangle_sides.1, side3: triangle_sides.2 };

  println!("Area of rectangle of height - {} and width - {} is {}", rect_height, rect_width, rect.area());
  println!("Perimeter of rectangle of height - {} and width - {} is {}", rect_height, rect_width, rect.perimeter());

  println!("Area of square of side - {} is {}", square_size, sq.area());
  println!("Perimeter of square of side - {} is {}", square_size, sq.perimeter());

  println!("Area of circle of radius - {} is {:.3}", circle_radius, circle.area());
  println!("Circumference of circle of radius - {} is {:.3}", circle_radius, circle.circumference());

  println!("Area of triangle of sides - {}, {}, {} is {:.3}", triangle_sides.0, triangle_sides.1, triangle_sides.2,
    triangle.area());
}

struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn perimeter(&self) -> u32 {
    2 * (self.width + self.height)
  }

  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size}
  }
}

struct Circle {
  radius: f32
}

impl Circle {
  fn area(&self) -> f32 {
    f32::consts::PI * self.radius * self.radius
  }

  fn circumference(&self) -> f32 {
    2.0 * f32::consts::PI * self.radius
  }
}

struct Triangle {
  side1: f32,
  side2: f32,
  side3: f32
}

impl Triangle {
  fn area(&self) -> f64 {
    // Area = sqrt ( s ( s - a ) x ( s - b ) x  (s - c) ) where s = side1 + side2 + side3 / 2
    let s = (self.side1 + self.side2 + self.side3) / 2.0;
    ((s* (s - self.side1) * (s - self.side2) * (s - self.side3)) as f64).sqrt()
  }
}

