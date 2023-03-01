# file_parse

This repo has two binaries, file_parse and generate_file. Use the latter to generate a file with
a set amount of lines and optionally set the character length per line.

> **Note**
>
> [Learn more about generate_file here][(https://github.com/thesandybridge/random_file#readme) or run the command with `--help`.

Once the file has been generates run file_parse with the file path passed as an argument.

file_parse will attempt to do the following:

1. open the file and read each line to an array/vector
2. filter out all alphabetical characters per line
3. convert all numerical strings to integers
4. return the length of updated array

It will do this in 3 different languages JavaScript (node & bun), Python, Rust.

The end result should basically show you how many lines had numbers in them.
Generate file randomly generates an alphanumeric string for each line, so you should
see a different result every time you generate a new file.


