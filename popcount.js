function popcountReadOnly(arr) {
    console.time('popcountReadOnly');
    let sum = 0;
    for (let i = arr.length - 1; i >= 0; i--) {
        sum += arr[i];
    }
    console.timeEnd('popcountReadOnly');
    return sum;
}

function standardLoop(arr) {
    console.time('standardLoop');
    let sum = 0;
    for (let i = 0; i < arr.length; i++) {
        sum += arr[i];
    }
    console.timeEnd('standardLoop');
    return sum;
}

const nums1 = Array.from({ length: 1_000_000 }, (_, i) => i + 1);
const nums2 = [...nums1];

console.log("Running popcountReadOnly:");
const sum1 = popcountReadOnly(nums1);

console.log("\nRunning standardLoop:");
const sum2 = standardLoop(nums2);

console.log(`\nResults:\nPopcount Sum: ${sum1}\nStandard Loop Sum: ${sum2}`);
