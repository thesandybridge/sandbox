export function createArray(length: number): number[] {
    const arr = []

    for (let i = 0; i < length; i++) {
        arr.push(i)
    }

    return arr
}

export function sumUsingReduce(arr: number[]): number {
    return arr.reduce((a, b) => {return a + b}, 0)
}

export function sum(arr: number[]): number {
    let total = 0
    for (let i = 0; i < arr.length; i ++) {
        total += arr[i]
    }
    return total
}
