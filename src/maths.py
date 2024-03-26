from .error_handling import *
from .matrix_operations import *
from .args_handling import *

NB_ITER = 50

def compute_exp(matrix):
    try:
        result = get_identity_matrix(len(matrix))
        for n in range(1, NB_ITER):
            result = add_matrices(result, divide_matrix(power_matrix(matrix, n), math.factorial(n)))
        return result
    except:
        display_error("Cannot compute exponential, matrix is too large.")
        exit(84)

def compute_cos(matrix):
    try:
        result = get_identity_matrix(len(matrix))
        for n in range(1 ,NB_ITER):
            if n % 2 == 0:
                result = add_matrices(result, divide_matrix(power_matrix(matrix, 2 * n), math.factorial(2 * n)))
            else:
                result = substract_matrices(result, divide_matrix(power_matrix(matrix, 2 * n), math.factorial(2 * n)))
        return result
    except:
        display_error("Cannot compute cos, matrix is too large.")
        exit(84)


def compute_sin(matrix):
    try:
        result = matrix
        for n in range(2, NB_ITER):
            if n % 2 == 0:
                result = add_matrices(result, divide_matrix(power_matrix(matrix, 2 * n + 1), math.factorial(2 * n + 1)))
            else:
                result = substract_matrices(result, divide_matrix(power_matrix(matrix, 2 * n + 1), math.factorial(2 * n + 1)))
        return result
    except:
        display_error("Cannot compute sin, matrix is too large.")
        exit(84)


def compute_cosh(matrix):
    try:
        result = get_identity_matrix(len(matrix))
        for n in range(1, NB_ITER):
            result = add_matrices(result, divide_matrix(power_matrix(matrix, 2 * n), math.factorial(2 * n)))
        return result
    except:
        display_error("Cannot compute cosh, matrix is too large.")
        exit(84)

def compute_sinh(matrix):
    try:
        result = matrix
        for n in range(1, NB_ITER):
            result = add_matrices(result, divide_matrix(power_matrix(matrix, 2 * n + 1), math.factorial(2 * n + 1)))
        return result
    except:
        display_error("Cannot compute sinh, matrix is too large.")
        exit(84)


def detect_function(args):
    float_args = args_to_float(args)
    try:
        base_matrix = create_matrix(float_args)
    except:
        display_error("Cannot create matrix, invalid values.")
        exit(84)
    match args[1]:
        case "EXP":
            result = compute_exp(base_matrix)
        case "COS":
            result = compute_cos(base_matrix)
        case "SIN":
            result = compute_sin(base_matrix)
        case "COSH":
            result = compute_cosh(base_matrix)
        case "SINH":
            result = compute_sinh(base_matrix)
        case _:
            display_error("Unknown fun argument \"{}\", please refer to help for known fun.".format(args[0]))
            exit(84)
    display_matrix(result)