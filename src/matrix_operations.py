import math
from .error_handling import *

def get_identity_matrix(size):
    return [[1 if i == j else 0 for j in range(size)] for i in range(size)]

def create_matrix(args):
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

def multiply_matrix_nb(matrix, nb):
    result = []

    for i in range(len(matrix)):
        line = []
        for j in range(len(matrix)):
            line.append(matrix[i][j] * nb)
        result.append(line)
    return result

def add_matrices(matrix_a, matrix_b):
    result = []

    for i in range(len(matrix_a)):
        line = []
        for j in range(len(matrix_a[i])):
            line.append(matrix_a[i][j] + matrix_b[i][j])
        result.append(line)
    return result

def substract_matrices(matrix_a, matrix_b):
    result = []

    for i in range(len(matrix_a)):
        line = []
        for j in range(len(matrix_a[i])):
            line.append(matrix_a[i][j] - matrix_b[i][j])
        result.append(line)
    return result

def multiply_matrices(matrix_a, matrix_b):
    result = []

    for i in range(len(matrix_a)):
        row = []
        for j in range(len(matrix_b[0])):
            value = 0
            for k in range(len(matrix_a[i])):
                value += (matrix_a[i][k] * matrix_b[k][j])
            row.append(value)
        result.append(row)
    return result

def power_matrix(matrix, power):
    result = matrix

    for i in range(power - 1):
        result = multiply_matrices(result, matrix)
    return result

def divide_matrix(matrix, nb):
    result = []

    for i in range(len(matrix)):
        line = []
        for j in range(len(matrix[i])):
            line.append(matrix[i][j] / nb)
        result.append(line)
    return result

def display_matrix(matrix):
    for line in matrix:
        for i in range(len(matrix) - 1):
            print("{:.2f}".format(line[i]), end="\t")
        print("{:.2f}".format(line[-1]))