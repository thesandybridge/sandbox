export function spreading(arr: []): [] {
    return [...arr]
}

export function usingPush(arr: []): number[] | string[] {
    const items: number[] | string[] = []

    for (let i = 0; i < arr.length; i++) {
        items.push(arr[i])
    }

    return items
}
