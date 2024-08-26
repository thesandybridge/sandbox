function weird() {
    let buffer = new ArrayBuffer(100_000_000)

    const func = () => {
        console.log(buffer.byteLength)
    }
    func()
    buffer = null
}

weird()
