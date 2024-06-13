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
        const newNode = new Node(value)

        if (this.head === null) {
            this.head = newNode
        } else {
            let current = this.head
            while (current.next !== null) {
                current = current.next
            }
            current.next = newNode
        }
        this.size++
    }

    remove(value) {
        if (!this.head) return null
        if (this.head.value === value) {
            this.head = this.head.next
        } else {
            let current = this.head
            while (current.next && current.next.value !== value) {
                current = current.next
            }
            if (current.next) {
                current.next = current.next.next
            }
        }
        this.size--
    }

    get(idx) {
        return this.#getAt(idx).value
    }

    isEmpty() {
        return this.size === 0
    }

    getSize() {
        return this.size
    }

    #getAt(idx) {
        let current = this.head
        for (let i = 0; current && i < idx; ++i) {
            current = current.next
        }
        return current
    }
}

const list = new LinkedList()

list.add("hello")
list.add("matt")
list.add("kurt")
list.add("usa")

console.log(list.get(0))
