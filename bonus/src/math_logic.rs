use crate::error_handling::display_error;
use std::process;

fn args_to_float(args: &Vec<String>) -> Vec<f64>{
    args[2..].iter().map(|x| match x.parse::<f64>() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid argument \"{}\", all matrix coefficients must be numbers.", x);
            process::exit(84);
        }
    }).collect()
}

fn get_identity_matrix(size: usize) -> Vec<Vec<u8>> {
    (0..size).map(|i| {
        (0..size).map(|j| {
            if i == j {1} else {0}
        }).collect()
    }).collect()
}

fn create_matrix(float_args: Vec<f64>) -> Vec<Vec<f64>> {
    let matrix_size = (float_args.len() as f64).sqrt() as usize;

    if matrix_size.pow(2) != float_args.len() {
        display_error("Matrix is not square, matrix must be square.");
        process::exit(84);
    }
    float_args.chunks(matrix_size).map(|chunk| chunk.to_vec()).collect()
}

fn compute_exp(base_matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let result = get_identity_matrix(base_matrix.len());
    for n in 1..50 {
        result = add_matrices(result, divide_matrix(power_matrix(matrix, n), ));
    }//might replace this later with NB_ITER 
}

fn compute_cos(base_matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    for line in base_matrix {
        print!("Line:");
        for element in line {
            print!("{}, ", element);
        }
        println!("");
    }
}

fn compute_sin(base_matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    for line in base_matrix {
        print!("Line:");
        for element in line {
            print!("{}, ", element);
        }
        println!("");
    }
}

fn compute_cosh(base_matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    for line in base_matrix {
        print!("Line:");
        for element in line {
            print!("{}, ", element);
        }
        println!("");
    }
}

fn compute_sinh(base_matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    
}

fn display_matrix(matrix: Vec<Vec<f64>>) {
    for line in matrix {
        print!("Line:");
        for element in line {
            print!("{}, ", element);
        }
        println!("");
    }
}

pub fn detect_function(args: &Vec<String>) {
    let float_args = args_to_float(args);
    let base_matrix = create_matrix(float_args);

    let result = match args[1].as_str() {
        "EXP" => {compute_exp(&base_matrix)}
        "COS" => {compute_cos(&base_matrix)}
        "SIN" => {compute_sin(&base_matrix)}
        "COSH" => {compute_cosh(&base_matrix)}
        "SINH" => {compute_sinh(&base_matrix)}
        _ => {
            display_error(format!("Unknown fun argument \"{}\", please refer to help for known fun.", args[0]).as_str());
            process::exit(84);
        }
    };
    display_matrix(result);
}