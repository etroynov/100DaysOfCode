package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Enter your str: ")
	input, _ := reader.ReadString('\n')

	fmt.Println("Your string: ", input)
}
