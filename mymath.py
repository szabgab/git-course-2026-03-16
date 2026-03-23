def add(op, *args):
    if op == "+":
        result = 0
        for val in args:
            result += val
        return result
    if op == "*":
        result = 1
        for val in args:
            result *= val
        return result

