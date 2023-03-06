import sys

args = sys.argv
depth = sys.getrecursionlimit()

def count_to_rec(n, c):
    """
    Add 1 to n until it is equal to c using recursion. 

    :param n int: number to start at
    :param c int: count ceiling
    """
    if c >= (depth - 2):
        print(f"{c} exceeds maximum recursion depth, picker a number less than {c}")
        return
    if n == c:
        return
    n+=1
    print(n)
    count_to_rec(n, c)

def count_to(n, c):
    """
    Add 1 to n until it is equal to c. 

    :param n int: number to start at
    :param c int: count ceiling
    """
    while n <= c:
        print(n)
        n+=1
    

if args[1] == "-r":
    num = int(args[2])
    count = int(args[3])
    count_to_rec(num, count)
    print("used recursion")

else:
    num = int(args[1])
    count = int(args[2])
    count_to(num, count)
