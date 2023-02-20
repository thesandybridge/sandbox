const fs = require("fs")
const result = fs.readFileSync("../stuff.txt").toString().split("\n")
console.log(result)

