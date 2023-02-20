use num_format::{Locale, ToFormattedString};
use std::time::Instant;

use rand::Rng;

fn main() {
    let mut iterations: i64 = 10_000_000;
    loop {
        run(iterations);
        iterations *= 10;
    }
}

fn run(iterations: i64) {
    let mut num_circle = 0.0;
    let mut num_total = 0.0;
    let mut rng = rand::thread_rng();
    let now = Instant::now();
    for _ in 0..iterations {
        let x: f64 = rng.gen_range(0.0..1.0);
        let y: f64 = rng.gen_range(0.0..1.0);
        let distance: f64 = (x * x) + (y * y);
        if distance <= 1.0 {
            num_circle += 1.0;
        }
        num_total += 1.0;
    }
    let pi = 4.0 * (num_circle / num_total);
    let elapsed_time = now.elapsed();
    println!("Calculated value of pi is: {}", pi);
    println!(
        "Time taken: {} seconds for {} iterations.",
        elapsed_time.as_secs_f32(),
        iterations.to_formatted_string(&Locale::en)
    );
}
