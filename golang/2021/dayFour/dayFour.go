package dayfour

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type gridS struct {
	Grid  [][]string
	Check [][]bool
	Score int
	Round int
}

func openFile(filename string) bufio.Scanner {
	f, err := os.Open(fmt.Sprintf("dayFour/%s", filename))

	if err != nil {
		panic(err)
	}

	return *bufio.NewScanner(f)
}

func First() {
	buf := openFile("input.txt")

	buf.Scan()

	firstLine := buf.Text()

	buf.Scan()

	inputs := strings.Split(firstLine, ",")

	var grid [][]string

	minRounds := 0
	first := true
	score := 0

	for buf.Scan() {
		if buf.Text() == "" {
			g := &gridS{Grid: grid, Round: 0, Score: 0, Check: createCheckArray()}

			g.play(inputs)

			if first || g.Round < minRounds {
				score = g.Score
				minRounds = g.Round
				first = false
			}

			buf.Scan()

			grid = make([][]string, 0)
		}

		grid = append(grid, formatRow(buf.Text()))
	}

	g := &gridS{Grid: grid, Round: 0, Score: 0, Check: createCheckArray()}

	g.play(inputs)

	if g.Round < minRounds {
		score = g.Score
	}

	fmt.Printf("Best Score : %d\n", score)
}

func Second() {
	buf := openFile("input.txt")

	buf.Scan()

	firstLine := buf.Text()

	buf.Scan()

	inputs := strings.Split(firstLine, ",")

	var grid [][]string

	maxRound := 0
	score := 0

	for buf.Scan() {
		if buf.Text() == "" {
			g := &gridS{Grid: grid, Round: 0, Score: 0, Check: createCheckArray()}

			g.play(inputs)

			if g.Round > maxRound {
				score = g.Score
				maxRound = g.Round
			}

			buf.Scan()

			grid = make([][]string, 0)
		}

		grid = append(grid, formatRow(buf.Text()))
	}

	g := &gridS{Grid: grid, Round: 0, Score: 0, Check: createCheckArray()}

	g.play(inputs)

	if g.Round > maxRound {
		score = g.Score
	}

	fmt.Printf("Best Score : %d\n", score)
}

func (g *gridS) play(draw []string) {
	g.sum()

	var lastNb int

	for _, d := range draw {
		for x := 0; x < 5; x++ {
			for y := 0; y < 5; y++ {
				if !g.isChecked(x, y) {
					if d == g.get(x, y) {
						nb, _ := strconv.Atoi(d)

						g.Check[x][y] = true
						g.Score -= nb
						lastNb = nb
					}
				}
			}
		}
		g.Round++

		if g.isWin() {
			break
		}
	}

	g.Score *= lastNb
}

func (grid *gridS) get(x, y int) string {
	return string(grid.Grid[x][y])
}

func (grid *gridS) isChecked(x, y int) bool {
	return grid.Check[x][y]
}

func createCheckArray() [][]bool {
	arr := make([][]bool, 5)

	for x := 0; x < 5; x++ {
		temp := []bool{false, false, false, false, false}

		arr[x] = temp
	}

	return arr
}

func (g *gridS) isWin() bool {
	for _, b := range g.Check {
		if b[0] && b[1] && b[2] && b[3] && b[4] {
			return true
		}
	}

	for i := 0; i < 5; i++ {
		if g.Check[0][i] && g.Check[1][i] && g.Check[2][i] && g.Check[3][i] && g.Check[4][i] {
			return true
		}
	}

	return false
}

func (grid *gridS) sum() {
	for _, s := range grid.Grid {
		for _, c := range s {
			nb, _ := strconv.Atoi(c)
			grid.Score += nb
		}
	}
}

func formatRow(row string) []string {
	newRow := make([]string, 0)

	split := strings.Split(row, " ")

	for _, s := range split {
		if s == "" {
			continue
		}

		newRow = append(newRow, s)
	}

	return newRow
}
