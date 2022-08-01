def factorialDivide(n,k):
    output = 1
    for i in range (n - k):
        output *= n - i
    return output

def factorialSubtract(n, k):
    output = 1
    for i in range (n - k):
        output *= i + 1
    return output

def choose(n, k):
    return int(factorialDivide(n, k) / factorialSubtract(n, k))

print(choose(78, 9))