import time
import sys
args = sys.argv

start = time.time()
file = open(args[1], "r").read()

arr = [
    [int(num) for num in line if num.isdigit()] 
    for line in file.strip().split("\n")
]

result = [i for i in arr if len(i) > 0]

end = time.time()

print(result)
print(f"Exec Time: {(end - start) * 1000}ms")
print(f"Array Length: {len(result)}")
