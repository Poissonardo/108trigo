import math
from .error_handling import *

def create_matrix(args):
    import math
    matrix_size = int(math.sqrt(len(args)))
    args_index = 0
    matrix = []

    if math.pow(matrix_size, 2) != len(args):
        display_error("Matrix is not square, matrix must be square.")
        exit(84)
    for i in range(matrix_size):
        line = []
        for j in range(matrix_size):
            line.append(args[args_index])
            args_index += 1
        matrix.append(line)
    return matrix

def add_matrices(matrix_a, matrix_b):
    result = []

    for i in range(len(matrix_a)):
        row = []
        for j in range(len(matrix_a[i])):
            row.append(matrix_a[i][j] + matrix_b[i][j])
        result.append(row)
    return result

def substract_matrices(matrix_a, matrix_b):
    result = []

    for i in range(len(matrix_a)):
        row = []
        for j in range(len(matrix_a[i])):
            row.append(matrix_a[i][j] - matrix_b[i][j])
        result.append(row)
    return result

def multiply_matrices(matrix_a, matrix_b):
    result = []

    for i in range(len(matrix_a)):
        row = []
        for j in range(len(matrix_b[0])):
            value = 0
            for k in range(len(matrix_a[i])):
                value += matrix_a[i][k] * matrix_b[k][j]
            row.append(value)
        result.append(row)
    return result