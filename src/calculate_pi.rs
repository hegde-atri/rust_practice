use rand::Rng;

pub fn run(){
    println!("Calculate pi reached!");

    let mut num_circle = 0.0;
    let mut num_total = 0.0;
    let mut rng = rand::thread_rng();

    let userinput = get_input("Enter number of points");
    let i = userinput.parse::<i32>().unwrap();
    for _ in 0..i {
        let x: f64 = rng.gen_range(0.0..1.0);
        let y: f64 = rng.gen_range(0.0..1.0);
        let distance: f64 = (x*x) + (y*y);
        if distance<=1.0 {
            num_circle += 1.0;
        }
        num_total += 1.0;
    }
    let pi = 4.0 * (num_circle/num_total);
    println!("Calculated value of pi is: {}", pi);

}

fn get_input(prompt: &str) -> String {
    let mut rl = rustyline::Editor::<()>::new();
    println!("{}", prompt);
    let readline = rl.readline(">> ");
    match readline {
        Ok(line) => {
            line
        }
        Err(_) => {
            String::from("")
        }
    }
}