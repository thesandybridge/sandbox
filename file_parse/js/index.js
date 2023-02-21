import fs from "fs"
const args = process.argv

let st = performance.now()
const result = fs.readFileSync(args[2])
    .toString()
    .trim()
    .split("\n")
let et = performance.now()

console.log(`Exec Time: ${ et - st}ms`)

