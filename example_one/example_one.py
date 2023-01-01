import sys

def append_list_func(arg1):
    arr = [1,2,3,4,5,6,7,8,9,10]
    file = open(arg1,'r')
    lines = file.read()
    values = [int(line) for line in lines.split(",")]

    for value in values:
        arr.append(value)

    print(arr)

append_list_func(sys.argv[1])
