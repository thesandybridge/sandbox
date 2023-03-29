import random, sys

args = sys.argv

def generate_list(n):
    for _ in range(n):
        with open("list", "a") as f:
            f.write(f"{random.randint(0,10_000)}\n")


generate_list(args[1])





