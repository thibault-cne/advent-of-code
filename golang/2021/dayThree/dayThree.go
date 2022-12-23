package daythree

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

func First() {
	length := 12

	ones, zeros, _ := calculateArrays(length)

	gamma := 0
	epsilon := 0

	for i := 0; i < length; i++ {
		if ones[i] > zeros[i] {
			gamma += int(math.Pow(2, float64(length-(i+1))))
		} else {
			epsilon += int(math.Pow(2, float64(length-(i+1))))
		}
	}

	fmt.Printf("Result : %d\n", gamma*epsilon)
}

func calculateArrays(length int) ([]int, []int, []string) {
	ones := make([]int, length)
	zeros := make([]int, length)
	s := make([]string, 0)

	f, err := os.Open("dayThree/input.txt")

	if err != nil {
		fmt.Printf("Error on opening inputs : %s\n", err.Error())
		return nil, nil, nil
	}

	defer f.Close()

	buffer := bufio.NewScanner(f)

	for buffer.Scan() {
		for index, b := range buffer.Text() {
			if b == '1' {
				ones[index]++
			} else {
				zeros[index]++
			}
		}

		s = append(s, buffer.Text())
	}

	return ones, zeros, s
}

func calculateNewArrays(s []string, length int) ([]int, []int) {
	ones := make([]int, length)
	zeros := make([]int, length)

	for _, p := range s {
		for index, b := range p {
			if b == '1' {
				ones[index]++
			} else {
				zeros[index]++
			}
		}
	}

	return ones, zeros
}

func Second() {
	length := 12

	oxygen := byteToInt(calculateOxygenGenRating(length))
	dyoxygen := byteToInt(calculateDyoxygenScrubRating(length))

	fmt.Printf("Result : %d\n", oxygen*dyoxygen)
}

func calculateOxygenGenRating(length int) string {
	ones, zeros, s := calculateArrays(length)

	index := 0

	for len(s) != 1 && index < length {
		temp := make([]string, 0)

		if ones[index] >= zeros[index] {
			for _, p := range s {
				if p[index] == '1' {
					temp = append(temp, p)
				}
			}
		} else {
			for _, p := range s {
				if p[index] == '0' {
					temp = append(temp, p)
				}
			}
		}

		s = temp
		ones, zeros = calculateNewArrays(s, length)
		index++
	}

	return s[0]
}

func calculateDyoxygenScrubRating(length int) string {
	ones, zeros, s := calculateArrays(length)

	index := 0

	for len(s) != 1 && index < length {
		temp := make([]string, 0)

		if ones[index] >= zeros[index] {
			for _, p := range s {
				if p[index] != '1' {
					temp = append(temp, p)
				}
			}
		} else {
			for _, p := range s {
				if p[index] != '0' {
					temp = append(temp, p)
				}
			}
		}

		s = temp
		ones, zeros = calculateNewArrays(s, length)
		index++
	}

	return s[0]
}

func byteToInt(b string) int {
	length := len(b)
	result := 0

	for i, v := range b {
		if v == '1' {
			result += int(math.Pow(2, float64(length-(i+1))))
		}
	}

	return result
}
