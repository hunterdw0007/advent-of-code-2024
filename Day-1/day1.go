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
	file, _ := os.Open("data.txt")
	defer file.Close()

	firstNumbers := make([]int, 0)
	secondNumbers := make([]int, 0)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		numbers := strings.Fields(scanner.Text())

		first, _ := strconv.Atoi(numbers[0])
		second, _ := strconv.Atoi(numbers[1])

		insertSort(&firstNumbers, first)
		insertSort(&secondNumbers, second)
	}

	output := compareLists(&firstNumbers, &secondNumbers)

	fmt.Printf("Difference: %v\n", output)

	output = compareLists(&secondNumbers, &firstNumbers)

	fmt.Printf("Sanity Check: %v\n", output)
}

func insertSort(list *[]int, num int) {
	*list = append(*list, num)
	i := len((*list)) - 1
	for i > 0 && (*list)[i-1] > (*list)[i] {
		(*list)[i-1], (*list)[i] = (*list)[i], (*list)[i-1]
		i--
	}
}

func compareLists(first *[]int, second *[]int) int {
	totalDiff := 0
	for index := range *first {
		diff := (*first)[index] - (*second)[index]
		if diff < 0 {
			diff = -diff
		}
		totalDiff += diff
	}
	return totalDiff
}
