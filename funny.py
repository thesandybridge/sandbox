def isEven(n, size):
    nums = [i for i in range(size)]
    for num in nums:
        if n == num:
            if num % 2 == 0:
                return "even"
            else:
                return "false"
    return "idk, maybe it's too big?"

print(isEven(800_577, 1_000_000))
