def create_kitten(name, age, owner):
    return {
        "name": name,
        "age": age,
        "owner": owner
    }


owner = {
    "name": "Susan",
    "age": 61,
    "number_of_pets": 3
}

ginger = create_kitten("Ginger", 9, owner)
whiskey = create_kitten("Whiskey", 9, owner)

cats = []

cats.append(ginger)
cats.append(whiskey)

#print(cats[1]["owner"]["number_of_pets"])


with open('example', 'r') as file:
    lines = file.readlines()

lines = [line.strip() for line in lines]

print(lines[2])
