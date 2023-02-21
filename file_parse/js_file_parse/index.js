const fs = require("fs")
const result = fs.readFileSync("../stuff.txt").toString().trim().split("\n")
console.log(result)

