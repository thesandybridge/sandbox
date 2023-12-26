package main

import (
    "fmt"
    "time"
)

func formatTime(t time.Time) string {
    return t.Format("01-02-2006 15:04")
}

func handleInput(message string) string {
    fmt.Print(message)
    var input string
    fmt.Scanln(&input)
    return input
}

func main() {
    input := handleInput("What is your name? ")
    message := fmt.Sprintf("Hello, %s! The time is %s", input, formatTime(time.Now()))
    fmt.Println(message)
}
