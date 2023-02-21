import fs from "fs"
const args = process.argv

let st = performance.now()
const result = fs.readFileSync(args[2])
    .toString()
    .trim()
    .split("\n")

let numbers = result
    .map(s => s
        .split("")
        .filter(c => !isNaN(c))
        .map((c) => parseInt(c, 10))
    )
    .filter((s) => s.length > 0)

let et = performance.now()

console.log(`Exec Time: ${ et - st}ms`)
console.log("Array Length: ", numbers.length)

