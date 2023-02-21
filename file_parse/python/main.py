import sys
args = sys.argv

file = open(args[1], "r").read()

result = [line for line in file.strip().split("\n")]

print(result)
