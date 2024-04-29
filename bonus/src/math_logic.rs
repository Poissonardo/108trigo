use crate::error_handling::display_error;
use std::process;
use crate::matrix_manipulation::{add_matrices, divide_matrix, power_matrix, subtract_matrices};

/*
This is bad Rust code translated from Python as is because of lack of time
A proper implementation would contain specific data structures and methods to
manipulate matrix
*/

fn args_to_float(args: &Vec<String>) -> Vec<f64>{
    args[2..].iter().map(|x| match x.parse::<f64>() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid argument \"{}\", all matrix coefficients must be numbers.", x);
            process::exit(84);
        }
    }).collect()
}

fn get_identity_matrix(size: usize) -> Vec<Vec<f64>> {
    (0..size).map(|i| {
        (0..size).map(|j| {
            if i == j {1 as f64} else {0 as f64}
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

pub fn factorial(n: usize) -> f64 {
    let mut result = 1.0;
    for i in 1..=n {
        result *= i as f64;
    }
    result
}

fn compute_exp(base_matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result = get_identity_matrix(base_matrix.len());
    for n in 1..50 { //might replace this later with NB_ITER 
        result = add_matrices(&result, divide_matrix(&power_matrix(&base_matrix, n), factorial(n) as i32));
    }
    result
}

pub fn compute_cos(matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let nb_iter: usize = 10; // replace with your actual value
    let mut result = get_identity_matrix(matrix.len());

    for n in 1..nb_iter {
        let powered_matrix = power_matrix(&matrix, 2 * n);
        let divided_matrix = divide_matrix(&powered_matrix, factorial(2 * n) as i32);
        if n % 2 == 0 {
            result = add_matrices(&result, divided_matrix);
        } else {
            result = subtract_matrices(&result, &divided_matrix);
        }
    }
    result
}


pub fn compute_sin(matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let nb_iter: usize = 50; // might use NB_ITER later on
    let mut result = matrix.clone();

    for n in 1..nb_iter {
        let powered_matrix = power_matrix(&matrix, 2 * n + 1);
        let divided_matrix = divide_matrix(&powered_matrix, factorial(2 * n + 1) as i32);
        if n % 2 == 0 {
            result = add_matrices(&result, divided_matrix);
        } else {
            result = subtract_matrices(&result, &divided_matrix);
        }
    }
    result
}

pub fn compute_cosh(matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let nb_iter: usize = 10; // replace with your actual value
    let mut result = get_identity_matrix(matrix.len());

    for n in 1..nb_iter {
        let powered_matrix = power_matrix(&matrix, 2 * n);
        let divided_matrix = divide_matrix(&powered_matrix, factorial(2 * n) as i32);
        result = add_matrices(&result, divided_matrix);
    }

    result
}

pub fn compute_sinh(matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let nb_iter: usize = 10; // replace with your actual value
    let mut result = matrix.clone();

    for n in 1..nb_iter {
        let powered_matrix = power_matrix(&matrix, 2 * n + 1);
        let divided_matrix = divide_matrix(&powered_matrix, factorial(2 * n + 1) as i32);
        result = add_matrices(&result, divided_matrix);
    }
    result
}

pub fn display_matrix(matrix: &Vec<Vec<f64>>) {
    for line in matrix {
        for i in 0..(matrix.len() - 1) {
            print!("{:.2}\t", line[i]);
        }
        println!("{:.2}", line[matrix.len() - 1]);
    }
}

pub fn detect_function(args: &Vec<String>) {
    let float_args = args_to_float(args);
    let base_matrix = create_matrix(float_args);

    let result = match args[1].as_str() {
        "EXP" => {compute_exp(base_matrix)}
        "COS" => {compute_cos(base_matrix)}
        "SIN" => {compute_sin(base_matrix)}
        "COSH" => {compute_cosh(base_matrix)}
        "SINH" => {compute_sinh(base_matrix)}
        _ => {
            display_error(format!("Unknown fun argument \"{}\", please refer to help for known fun.", args[0]).as_str());
            process::exit(84);
        }
    };
    display_matrix(&result);
}