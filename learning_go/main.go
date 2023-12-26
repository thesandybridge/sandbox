package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Person struct {
	name string
	age  int
	job  string
}

func handleInput(message string) string {
	fmt.Print(message)
	reader := bufio.NewReader(os.Stdin)
	input, err := reader.ReadString('\n')
	if err != nil {
		fmt.Println("Error reading input:", err)
		return ""
	}
	return strings.TrimSpace(input)
}

func createPerson() Person {
	name := handleInput("What is your name? ")
	ageStr := handleInput("What is your age? ")
	age, err := strconv.ParseInt(ageStr, 10, 64)
	if err != nil {
		fmt.Println("Invalid input for age. Using default age 0.")
		age = 0
	}
	job := handleInput("What is your job? ")
	return Person{name, int(age), job}
}

func writePersonToFile(filepath string) {
	person := createPerson()
	file, err := os.OpenFile(filepath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0o644)
	if err != nil {
		fmt.Println("Error creating file:", err)
		return
	}
	defer file.Close()
	_, err = file.WriteString(fmt.Sprintf("%s,%d,%s\n", person.name, person.age, person.job))

	if err != nil {
		fmt.Println("Error writing to file:", err)
		return
	}
}

func main() {
	filepath := handleInput("Enter filepath: ")
	writePersonToFile(filepath)
}
