pub fn add_matrices(matrix_a: &Vec<Vec<f64>>, matrix_b: Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    let mut result = vec![];
    
    for i in 0..matrix_a.len() {
        let mut line = vec![];
        for j in 0..matrix_a.len() {
            line.push(matrix_a[i][j] + matrix_b[i][j]);
        }
        result.push(line);
    }
    result
}

pub fn divide_matrix(matrix: &Vec<Vec<f64>>, nb: i32) -> Vec<Vec<f64>> {
    let mut result = vec![];

    for i in 0..matrix.len() {
        let mut line = vec![];
        for j in 0..matrix.len() {
            line.push(matrix[i][j] / nb as f64);
        }
        result.push(line);
    }
    result
}

pub fn power_matrix(matrix: &Vec<Vec<f64>>, power: usize) -> Vec<Vec<f64>> {
    let mut result = matrix.clone();

    for _ in 0..(power - 1) {
        result = multiply_matrices(&result, &matrix);
    }

    result
}

pub fn multiply_matrices(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result = vec![vec![0.0; b[0].len()]; a.len()];

    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..b.len() {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    result
}

pub fn subtract_matrices(matrix_a: &Vec<Vec<f64>>, matrix_b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result = Vec::new();

    for i in 0..matrix_a.len() {
        let mut line = Vec::new();
        for j in 0..matrix_a[i].len() {
            line.push(matrix_a[i][j] - matrix_b[i][j]);
        }
        result.push(line);
    }

    result
}