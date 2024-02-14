import { stdin } from 'process';
import { inclusive_loop, exclusive_loop } from './scripts/loops';
import { fizzbuzz } from './scripts/fizzbuzz';

const readFromStdin = () => {
    return new Promise((resolve, reject) => {
        let inputData = ''
        stdin.on('data', chunk => {
            inputData += chunk
        })
        stdin.on('end', () => {
            resolve(inputData.trim())
        })
        stdin.on('error', err => {
            reject(err)
        })
    })
}

const processInput = async () => {
    try {
        const input = await readFromStdin()
        const number = parseInt(input as string, 10)
        if (isNaN(number)) {
            console.error('Please enter a valid number')
            return
        }
        console.log(inclusive_loop(number))
    } catch (err) {
        console.error('Error reading input:', err)
    }
}

processInput()
