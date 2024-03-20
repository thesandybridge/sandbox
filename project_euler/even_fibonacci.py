arr = []

def even_fibonacci(n):
    a, b = 0, 1
    while a < n:
        if a % 2 == 0:
            arr.append(a)
        a, b = b, a + b
    return sum(arr)

print(even_fibonacci(4000000))
