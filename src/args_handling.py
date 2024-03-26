from .error_handling import *

def args_to_float(args):
    try:
        result = [float(args[i]) for i in range(2, len(args))]
    except ValueError:
        display_error("Invalid argument, all matrix coefficients must be numbers.")
        exit(84)
    return result