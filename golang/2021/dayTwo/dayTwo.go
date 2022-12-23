package daytwo

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func First() {
	depth := 0
	horizontal := 0

	f, err := os.Open("dayTwo/input.txt")

	if err != nil {
		fmt.Printf("Error on opening inputs : %s\n", err.Error())
		return
	}

	defer f.Close()

	buffer := bufio.NewScanner(f)

	for buffer.Scan() {
		in := strings.Split(buffer.Text(), " ")
		x, _ := strconv.Atoi(in[1])

		switch in[0] {
		case "forward":
			horizontal += x
			break
		case "down":
			depth += x
			break
		case "up":
			depth -= x
			break
		}
	}

	fmt.Printf("Result : %d\n", horizontal*depth)
}

func Second() {
	depth := 0
	aim := 0
	horizontal := 0

	f, err := os.Open("dayTwo/input.txt")

	if err != nil {
		fmt.Printf("Error on opening inputs : %s\n", err.Error())
		return
	}

	defer f.Close()

	buffer := bufio.NewScanner(f)

	for buffer.Scan() {
		in := strings.Split(buffer.Text(), " ")
		x, _ := strconv.Atoi(in[1])

		switch in[0] {
		case "forward":
			horizontal += x
			depth += aim * x
			break
		case "down":
			aim += x
			break
		case "up":
			aim -= x
			break
		}
	}

	fmt.Printf("Result : %d\n", horizontal*depth)
}
