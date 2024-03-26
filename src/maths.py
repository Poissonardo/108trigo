from .error_handling import *
from .matrix_operations import *

NB_ITER = 100

def compute_exp(matrix):
    result = get_identity_matrix(len(matrix))
    for n in range(1, NB_ITER):
        result = add_matrices(result, divide_matrix(power_matrix(matrix, n), math.factorial(n)))
    return result

def compute_cos(args):
    args.pop(0)
    matrix = create_matrix(args)
    pass

def compute_sin(args):
    args.pop(0)
    matrix = create_matrix(args)
    pass

def compute_cosh(args):
    args.pop(0)
    matrix = create_matrix(args)
    pass

def compute_sinh(args):
    args.pop(0)
    matrix = create_matrix(args)
    pass

def detect_function(args):
    float_args = [float(args[i]) for i in range(2, len(args))]
    base_matrix = create_matrix(float_args)
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