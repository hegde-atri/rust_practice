pub fn get_input(prompt: &str) -> String {
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