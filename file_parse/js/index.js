import fs from "fs"
const args = process.argv

const result = fs.readFileSync(args[2])
    .toString()
    .trim()
    .split("\n")

console.log(result)

