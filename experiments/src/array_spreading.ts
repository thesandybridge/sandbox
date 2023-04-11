export function spread(arr1: number[], arr2: number[]): number[] {
    return [...arr1, ...arr2]
}

export function concat(arr1: number[], arr2: number[]): number[] {
    const arr = arr1.concat(arr2)
    return arr
}
