package main

import (
	"fmt"
	"io"
	"os"
)

func main() {
	fmt.Println("hi")
	file, err := os.Open("input.txt")

	input, err := io.ReadAll(file)

	levels := []
	
}
