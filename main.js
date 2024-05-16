// algorithm for comparing versions
function VersionCompare( version1, version2 ) {
    // requirements
    // return -1 when version1 < version2
    // 0 when v1 = v2
    // 1 v1 > v2

    // create arrays by splitting each string
    const v1Arr = version1.split('.')
    const v2Arr = version2.split('.')

    // store the larger of the two array lengths
    const maxLength = Math.max(v1Arr.length, v2Arr.length)

    for (let i = 0; i < maxLength; i++) {
        // rebuild the segments using 0 as a fill
        const num1 = i < v1Arr.length ? parseInt(v1Arr[i], 10) : 0
        const num2 = i < v2Arr.length ? parseInt(v2Arr[i], 10) : 0

        if (num1 > num2) {
            return 1
        }
        if (num1 < num2) {
            return -1
        }
    }

    return 0

}

console.log(VersionCompare("1.0.3", "1.0.4.5"))

function VersionCompareWithPrefix(version1, version2) {
    // Helper function to compare non-numeric pre-release parts
    function comparePreRelease(v1, v2) {
        if (v1 === v2) return 0;
        if (v1 === '') return 1;
        if (v2 === '') return -1;

        const preReleaseOrder = { 'alpha': 1, 'beta': 2, 'rc': 3 };
        const pr1 = preReleaseOrder[v1] || 0;
        const pr2 = preReleaseOrder[v2] || 0;

        if (pr1 === pr2) return 0;
        return pr1 > pr2 ? 1 : -1;
    }

    // Splitting the version into numeric and pre-release parts
    const regex = /^(\d+\.\d+\.\d+)(-(alpha|beta|rc))?$/;
    const match1 = version1.match(regex);
    const match2 = version2.match(regex);

    // Parse the main version and pre-release parts
    const v1Arr = match1[1].split('.');
    const v2Arr = match2[1].split('.');
    const v1Pre = match1[3] || '';
    const v2Pre = match2[3] || '';

    const maxLength = Math.max(v1Arr.length, v2Arr.length);

    // First compare the main version numbers
    for (let i = 0; i < maxLength; i++) {
        const num1 = i < v1Arr.length ? parseInt(v1Arr[i], 10) : 0;
        const num2 = i < v2Arr.length ? parseInt(v2Arr[i], 10) : 0;

        if (num1 > num2) {
            return 1;
        }
        if (num1 < num2) {
            return -1;
        }
    }

    // If main versions are equal, compare pre-release versions
    if (v1Pre || v2Pre) {
        const preResult = comparePreRelease(v1Pre, v2Pre);
        if (preResult !== 0) {
            return preResult;
        }
    }

    return 0;
}

console.log(VersionCompareWithPrefix("1.0.0-alpha", "1.0.0-beta")); // Outputs: -1
console.log(VersionCompareWithPrefix("1.0.0-rc", "1.0.0-alpha"));  // Outputs: 1
console.log(VersionCompareWithPrefix("1.0.0", "1.0.0"));          // Outputs: 0

