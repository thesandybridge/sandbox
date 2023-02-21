file = open("../stuff.txt", "r")
lines = file.read()
result = [line for line in lines.strip().split("\n")]
print(result)
