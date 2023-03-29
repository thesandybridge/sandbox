import sys, locale
args = sys.argv
locale.setlocale( locale.LC_ALL, '' )

def value(demand, curve):
    return demand * curve

def demand(jobs, supply):
    return jobs / supply

def average_time(times):
    sum_ = sum(times)
    avg_ = sum_ / len(times)

    return avg_

def calc_curve(time, average):
    return time / average

def percentage(v, n, sub=False):
    if sub == False:
        return v + (v * n / 100)
    return v - (v * n / 100)


def salary_potential(average, value):
    base = average
    if value < 1:
        base = percentage(average, 15, sub=True)
    elif value == 1:
        base = average
    else:
        base = percentage(average, value)

    return locale.currency(base, grouping=True)


jobs = int(args[1])
hours = int(args[2])
salary = int(args[3])

lines = open("list", "r").read()
times = [int(n) for n in lines.strip().split("\n")]
supply = len(times)

average = average_time(times)
dem = demand(jobs, supply)
curve = calc_curve(hours, average)
val = value(dem, curve)

potential = salary_potential(salary, val)

print(f"Supply: {supply}")
print(f"Jobs: {jobs}")
print(f"Hours: {hours}")
print(f"Value: {val}")
print("--------")
print(f"Salary Potential: {potential}")

