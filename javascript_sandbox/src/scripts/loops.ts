/**
 * Return an array of numbers from 1 to n, inclusive
 *
 * @param n the size of the array
 * @returns number[] an array of numbers from 1 to n, inclusive
 */
function inclusive_loop(n: number) {
    const arr = [];
    for (let i = 1; i <= n; ++i) {
        arr.push(i);
    }
    return arr;
}

/**
 * Return an array of numbers from 1 to n, exclusive
 *
 * @param n the size of the array
 * @returns number[] an array of numbers from 1 to n, exclusive
 */
function exclusive_loop(n: number) {
    const arr = [];
    for (let i = 1; i < n; i++) {
        arr.push(i);
    }
    return arr;
}

export { inclusive_loop, exclusive_loop };
