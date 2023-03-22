const fs = require("fs")

let lines = fs.readFileSync("list").toString().trim().split("\n")

let st = performance.now()
let items = []

function is_alphabetic(c) {
    return c.toUpperCase() != c.toLowerCase()
}

function checkUsingTrick(arr) {
    lines.forEach((line) => {
        Array.from(line).forEach(c => is_alphabetic(c) ? arr.push(c) : -1)
    })
}


checkUsingTrick(items)
let et = performance.now()
console.log(et - st)
