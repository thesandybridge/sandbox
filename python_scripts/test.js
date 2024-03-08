function modify_things(thing) {
    for (let i = 0; i < thing.length; i++) {
        thing[i] = i;
    }
}

let thing = new Array(1000000);
modify_things(thing);
let thing2 = thing.slice(0, 10);
console.log(thing);
console.log(thing2);
