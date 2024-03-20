def largest_palindrome(n, m):
    arr = []
    for i in range(10 ** (n - 1), 10 ** n):
        for j in range(10 ** (m - 1), 10 ** m):
            product = i * j
            if str(product) == str(product)[::-1]:
                arr.append(product)
    return max(arr)

print(largest_palindrome(3, 3))

