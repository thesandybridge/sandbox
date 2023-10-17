import sys
import re

args = sys.argv

file = open(args[1], "r")
lines = file.read()

pattern = re.compile(r'([a-zA-Z ,]+), \[([\d,]+)\]')
groups = []

for line in lines.split("\n"):
    match = pattern.match(line)
    if match:
        people = [person.strip() for person in match.group(1).split(", ")]
        weights = set(int(weight) for weight in match.group(2).split(","))
        groups.append((people, weights))

def redistribute(tables, table_weights, min_size):
    for under_idx, under_table in enumerate(tables):
        while len(under_table) < min_size:
            best_over_idx = None
            best_person = None

            for over_idx, over_table in enumerate(tables):
                if len(over_table) <= min_size:
                    continue

                common_weights = table_weights[under_idx].intersection(table_weights[over_idx])
                if common_weights:
                    for person in over_table:
                        best_over_idx = over_idx
                        best_person = person
                        break

                    if best_person:
                        break

            if best_person and best_over_idx is not None:
                under_table.append(best_person)
                tables[best_over_idx].remove(best_person)
            else:
                break

def distribute_to_tables(groups, num_tables, min_size, max_size):
    tables = [[] for _ in range(num_tables)]
    table_weights = [set() for _ in range(num_tables)]

    unseated = []

    for group, weights in sorted(groups, key=lambda x: len(x[0]), reverse=True):
        best_table_idx = None
        min_diff = float('inf')

        for j, table in enumerate(tables):
            current_size = len(table)
            diff = max_size - (current_size + len(group))

            if diff >= 0:
                if diff < min_diff:
                    min_diff = diff
                    best_table_idx = j

        if best_table_idx is not None:
            tables[best_table_idx].extend(group)
            table_weights[best_table_idx].update(weights)
        else:
            unseated.extend(group)

    redistribute(tables, table_weights, min_size)

    # Remove empty tables and print warning if any
    tables = [table for table in tables if table]
    if len(tables) < num_tables:
        print(f"Warning: Too many tables. Removed empty tables.")

    underfilled_tables = [i+1 for i, table in enumerate(tables) if len(table) < min_size]
    if underfilled_tables:
        print(f"Warning: Tables {underfilled_tables} are underfilled. Consider adding another table.")
        print(f"Unseated people: {unseated}")

    return tables

num_tables = int(args[2]) if len(args) > 2 and args[2].isdigit() else 6
min_size = 4
max_size = 7

tables = distribute_to_tables(groups, num_tables, min_size, max_size)

for i, table in enumerate(tables):
    print(f"Table {i + 1}: {table}")
    print(f"Total people: {len(table)}")

