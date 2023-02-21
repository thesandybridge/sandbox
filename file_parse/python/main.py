import time
import sys
args = sys.argv

start = time.time()
file = open(args[1], "r").read()

result = [line for line in file.strip().split("\n")]
end = time.time()

print(f"Exec Time: {(end - start) * 1000}ms")
