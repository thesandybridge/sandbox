class Queue {
    constructor() {
        this.items = []
    }

    enqueue(element) {
        this.items.push(element)
    }

    dequeue() {
        if (this.isEmpty()) {
            throw new Error("Queue is empty")
        }

        return this.items.shift()
    }

    front() {
        if (this.isEmpty()) {
            throw new Error("Queue is empty")
        }
        return this.items[0]
    }

    isEmpty() {
        return this.items.length === 0
    }

    size() {
        return this.items.length
    }

    clear() {
        this.items = []
    }
}
