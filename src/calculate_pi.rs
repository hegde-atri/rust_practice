use std::time::Instant;

use rand::Rng;

pub fn run(iterations: i32) {
    let mut num_circle = 0.0;
    let mut num_total = 0.0;
    let mut rng = rand::thread_rng();

    //let userinput = get_input("Enter number of points");
    // let i = userinput.parse::<i32>().unwrap();

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
    println!("Time taken: {} seconds.", elapsed_time.as_secs_f32());
}

// fn get_input(prompt: &str) -> String {
//     let mut rl = rustyline::Editor::<()>::new();
//     println!("{}", prompt);
//     let readline = rl.readline(">> ");
//     match readline {
//         Ok(line) => {
//             line
//         }
//         Err(_) => {
//             String::from("")
//         }
//     }
// }
