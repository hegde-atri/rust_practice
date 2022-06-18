mod calculate_pi;
mod common_functions;

use rustyline;
use rustyline::error::ReadlineError;

fn main() {
    let loop_active:bool = true;

    while loop_active{
        println!("\nA. Calculate pi");
        println!("Q. Quit \n");

        let choice = common_functions::get_input("\nEnter choice number").to_uppercase();
        let str_choice = choice.as_str();
        match str_choice {
            "A" => calculate_pi::run(),
            "Q" => break,
            _ => println!("Invalid")
        }
    }

}



fn get_num_value(value: String) -> i32 {
    let i = match value.parse::<i32>() {
        Ok(i) => i,
        Err(_) => -1
    };
    i
}
