file = open("../stuff.txt", "r")
lines = file.read()
result = [line for line in lines.split("\n")]
print(result)
