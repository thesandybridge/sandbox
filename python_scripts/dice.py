import random

user_input = input("Enter a dice size: ")

def roll(arr):
    # loop through the sides
    dice_length = len(arr)
    random_idx = random.randint(0, dice_length - 1)
    return arr[random_idx]

def make_dice():
    if user_input.isdigit():
        return list(range(1, int(user_input) + 1))
    else:
        print("Invalid input, please provide a number")

dice = make_dice()
dice_roll = roll(dice)

print(dice)
print(f"Results: {dice_roll}")


