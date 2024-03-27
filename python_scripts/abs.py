import sys

if len(sys.argv) != 2:
    print("enter a number")
    exit(1)

input = int(sys.argv[1])

def abs(x):
    mask = x >> 31
    return (x ^ mask) - mask

print(abs(input))
