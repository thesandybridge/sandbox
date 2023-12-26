package main

import (
    "fmt"
)

func main() {
    fmt.Print("Enter your name: ")
    var input string
    fmt.Scanln(&input)
    fmt.Println("Hello, " + input + "!")
}
