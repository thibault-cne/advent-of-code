package dayone

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var (
	first  int
	second int
)

func First() {
	f, err := os.Open("dayOne/input.txt")

	if err != nil {
		fmt.Printf("Error on opening inputs : %s\n", err.Error())
		return
	}

	defer f.Close()

	buffer := bufio.NewScanner(f)

	buffer.Scan()

	first, _ = strconv.Atoi(buffer.Text())

	cpt := 0

	for buffer.Scan() {
		second, _ = strconv.Atoi(buffer.Text())

		if second > first {
			cpt++
		}

		first = second
	}

	fmt.Printf("Cpt : %d\n", cpt)
}

func Second() {
	f, err := os.Open("dayOne/input.txt")

	if err != nil {
		fmt.Printf("Error on opening inputs : %s\n", err.Error())
		return
	}

	defer f.Close()

	buffer := bufio.NewScanner(f)

	arr := make([]int, 4)
	index := 1
	cpt := 0
	firstTurn := false

	for buffer.Scan() {
		nb, _ := strconv.Atoi(buffer.Text())

		switch index {
		case 1:
			arr[0] += nb
			break
		case 2:
			arr[0] += nb
			arr[1] += nb
			break
		case 3:
			arr[0] += nb
			arr[1] += nb
			arr[2] += nb

			if firstTurn {
				if arr[0] > arr[3] {
					cpt++
				}

				arr[3] = 0
			}

			firstTurn = true
			break
		case 4:
			arr[1] += nb
			arr[2] += nb
			arr[3] += nb

			if arr[1] > arr[0] {
				cpt++
			}

			arr[0] = 0
			break
		case 5:
			arr[0] += nb
			arr[2] += nb
			arr[3] += nb

			if arr[2] > arr[1] {
				cpt++
			}

			arr[1] = 0
			break
		case 6:
			arr[0] += nb
			arr[1] += nb
			arr[3] += nb

			if arr[3] > arr[2] {
				cpt++
			}

			arr[2] = 0
			index = 2
			break
		}

		index++
	}

	fmt.Printf("Cpt : %d\n", cpt)
}
