const totalItems = 103
const limit = 10
const page = 3

const totalPages = Math.ceil(totalItems / limit)

const startPage = 1
const lastPage = totalPages

const buildPagination = (delta) => {
    let pageArray = []

    // Add the first page
    pageArray.push(startPage)

    // Add the first `delta` pages in close order
    for (let i = 1; i < delta; i++) {
        if (page + i < totalPages) {
            pageArray.push(page + i)
        }
    }

    // Calculate the middle page between the current page and the last page
    const middlePage = Math.floor((page + lastPage) / 2)

    // Add ellipsis if necessary before middle page
    if (pageArray[pageArray.length - 1] < middlePage - 1) {
        pageArray.push("...")
    }

    // Add the middle page
    if (!pageArray.includes(middlePage) && middlePage > page) {
        pageArray.push(middlePage)
    }

    // Add ellipsis if necessary after middle page
    if (middlePage < lastPage - 1) {
        pageArray.push("...")
    }

    // Add the last page
    if (!pageArray.includes(lastPage)) {
        pageArray.push(lastPage)
    }

    return pageArray
}

const pageArray = buildPagination(3)
console.log(pageArray)
