import os

path = os.path.normpath(".")

files_number = 0
lines_number = 0

for dirpath, dirnames, filenames in os.walk(path):
    for filename in filenames:
        current_path = os.path.join(dirpath, filename)
        if current_path.endswith(".rs"):
            files_number += 1
            with open(current_path, encoding="utf8") as input_file:
                for line in input_file:
                    if line[:-1] != "":
                        lines_number += 1
print(files_number, lines_number)
