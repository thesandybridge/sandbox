file = open("../stuff.txt", "r").read()

result = [line for line in file.strip().split("\n")]

print(result)
