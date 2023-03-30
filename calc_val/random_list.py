import sys
from scipy.stats import truncnorm

def get_truncated_normal(mean=0, sd=1, low=0, upp=10):
    return truncnorm(
        (low - mean) / sd, (upp - mean) / sd, loc=mean, scale=sd)

args = sys.argv
length = int(args[1])

x = get_truncated_normal(mean=5000, sd=5000, low=0, upp=10000)
data = x.rvs(length)

def generate_list():
    for i in range(len(data)):
        with open("list", "a") as f:
            f.write(f"{round(data[i])}\n")

generate_list()
