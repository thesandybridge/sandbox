import { sum, sumUsingReduce } from "./sum"
import { spread, concat } from "./array_spreading"

function time(count: number, arr: number[], fn: (arr: number[]) => number) {
    const start = performance.now()
    for (let i = 0; i < count; ++i) {
        fn(arr)
    }
    return performance.now() - start
}

function timeTwo(count: number, arr: number[], fn: (arr: number[], arr2: number[]) => number[]) {
    const start = performance.now()
    for (let i = 0; i < count; ++i) {
        fn(arr, arr)
    }
    return performance.now() - start
}

function createArray(length: number): number[] {
    const arr = []

    for (let i = 0; i < length; i++) {
        arr.push(i)
    }

    return arr
}
const small = createArray(10)
const medium = createArray(100)
const large = createArray(1000)
const huge = createArray(10000)
const massive = createArray(1_000_000)

function testingSum(): object {


    const runs = [1, 10, 100, 1000]

    const results = {
        small: { sumUsingReduce: [] as number[], sum: [] as number[] },
        medium: { sumUsingReduce: [] as number[], sum: [] as number[] },
        large: { sumUsingReduce: [] as number[], sum: [] as number[] },
        huge: { sumUsingReduce: [] as number[], sum: [] as number[] },
        massive: { sumUsingReduce: [] as number[], sum: [] as number[] },
    }

    time(100, small, sumUsingReduce)
    time(100, small, sum)
    for (let i = 0; i < runs.length; ++i) {
        console.log("Starting test", runs[i])
        const run = runs[i]
        results.small.sumUsingReduce.push(time(run, small, sumUsingReduce))
        results.small.sum.push(time(run, small, sum))
        results.medium.sumUsingReduce.push(time(run, medium, sumUsingReduce))
        results.medium.sum.push(time(run, medium, sum))
        results.large.sumUsingReduce.push(time(run, large, sumUsingReduce))
        results.large.sum.push(time(run, large, sum))
        results.huge.sumUsingReduce.push(time(run, huge, sumUsingReduce))
        results.huge.sum.push(time(run, huge, sum))
        results.massive.sumUsingReduce.push(time(run, massive, sumUsingReduce))
        results.massive.sum.push(time(run, massive, sum))
    }

    return results
}

// console.log(testingSum())

function spreadVsConcat(): number[] {
    const spreaded = timeTwo(1000, massive, spread)
    const concated = timeTwo(1000, massive, concat)

    return [spreaded, concated]
}

const [spreaded, concated] = spreadVsConcat()

console.log(spreaded, concated)

