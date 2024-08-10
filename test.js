class Node {
    constructor(value) {
        this.value = value
        this.next = null
    }
}

class LinkedList {
    constructor() {
        this.head = null
        this.size = 0
    }

    add(value) {
        const node = new Node(value)

        if (!this.head) {
            this.head = node
        } else {
            let current = this.head
            while(current.next) {
                current = current.next
            }
            current.next = node
            this.size++
        }
    }

    insertAt(value, index) {
        if (index < 0 || index > this.size) {
            return 'Index out of bounds'
        }

        const node = new Node(value)

        if (index === 0) {
            node.next = this.head
            this.head = node
        } else {
            let current = this.head
            let previous = null
            let currentIndex = 0

            while (currentIndex < index) {
                previous = current
                current = current.next
                currentIndex++
            }

            node.next = current
            previous.next = node
        }

        this.size++
    }

    removeAt(index) {
        if (index < 0 || index >= this.size) {
            return 'Index out of bounds'
        }

        let current = this.head
        let previous = null
        let currentIndex = 0

        if (index === 0) {
            this.head = current.next
        } else {
            while (currentIndex < index) {
                previous = current
                current = current.next
                currentIndex++
            }

            previous.next = current.next
        }

        this.size--
        return current.value
    }

    getAt(index) {
        if (index < 0 || index >= this.size) {
            return 'Index out of bounds'
        }

        let current = this.head
        let currentIndex = 0

        while (currentIndex < index) {
            current = current.next
            currentIndex++
        }

        return current.value
    }

    isEmpty() {
        return this.size === 0
    }

    getSize() {
        return this.size
    }

    clear() {
        this.head = null
        this.size = 0
    }

    printList() {
        let current = this.head
        let result = ''
        while (current) {
            if (typeof current.value === 'object' && current.value !== null) {
                result += JSON.stringify(current.value)
            } else {
                result += current.value
            }
            if (current.next) {
                result += '->'
            }
            current = current.next
        }
        console.log(result.trim())
    }
}

const ALPHABET = Array.from({ length: 26 }, (_, i) => String.fromCharCode(65 + i))

const lookupTable = ALPHABET.reduce((acc, letter) => {
    acc[letter] = new LinkedList()
    return acc
}, {})

function addUser(name, age) {
    const firstLetter = name.charAt(0).toUpperCase()
    if (lookupTable[firstLetter]) {
        lookupTable[firstLetter].add({ name, age })
    } else {
        console.log(`No linked list found for letter ${firstLetter}`)
    }
}

addUser('Alice', 25)
addUser('Bob', 30)
addUser('Aaron', 20)
addUser('Charlie', 35)
addUser('Zelda', 300)
addUser('Elvis', 10)
addUser('George', 3)
addUser('Aurelius', 2)
addUser('Sol', 2)
addUser('Kelsie', 31)
addUser('Matt', 30)

ALPHABET.forEach((letter) => {
    console.log(`Users starting with ${letter}:`)
    lookupTable[letter].printList()
})
