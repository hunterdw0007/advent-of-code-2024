package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Day 1")
	file, _ := os.Open("../data.txt")
	defer file.Close()

	firstNumbers := make([]int, 0)
	secondNumbers := make(map[int]int)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		numbers := strings.Fields(scanner.Text())

		first, _ := strconv.Atoi(numbers[0])
		second, _ := strconv.Atoi(numbers[1])

		firstNumbers = append(firstNumbers, first)
		numberCounts(&secondNumbers, second)
	}

	//fmt.Printf("%v\n", secondNumbers)

	output := compareLists(&firstNumbers, &secondNumbers)

	fmt.Printf("Similarity Score: %v\n", output)
}

func numberCounts(list *map[int]int, num int) {
	if (*list)[num] == 0 {
		(*list)[num] = 1
		return
	}
	if (*list)[num] != 0 {
		(*list)[num]++
		return
	}
}

func compareLists(list *[]int, counts *map[int]int) int {
	//TODO
	return 0
}
