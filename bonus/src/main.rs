use std::env;
use std::process;

use math_logic::detect_function;
mod error_handling;
use error_handling::display_error;
mod math_logic;
mod matrix_manipulation;

fn main() {
    let args: Vec<String> = env::args().collect();
    let nb_args = args.len();
    
    if nb_args == 2 && args[1] == "-h".to_owned() {
        println!("USAGE\n\t./108trigo fun a0 a1 a2 ...\nDESCRIPTION\n\tfun\tfunction to be applied, among at least \"EXP\", \"COS\", \"SIN\", \"COSH\" and \"SINH\"\n\tai\tcoeficients of the matrix");
        process::exit(0);
    } else if nb_args < 3 {
        display_error("Not enough arguments, takes a minimum of three arguments.");
        process::exit(84);
    }
    detect_function(&args);
}
