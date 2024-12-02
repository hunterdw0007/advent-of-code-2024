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
	numberCountMap := make(map[int]int)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		numbers := strings.Fields(scanner.Text())

		first, _ := strconv.Atoi(numbers[0])
		second, _ := strconv.Atoi(numbers[1])

		insertSort(&firstNumbers, first)
		insertSort(&secondNumbers, second)
		numberCounts(&numberCountMap, second)
	}

	output := compareLists(&firstNumbers, &secondNumbers)

	fmt.Printf("Total Difference: %v\n", output)

	output = compareMap(&firstNumbers, &numberCountMap)

	fmt.Printf("Similarity Score: %v\n", output)
}

// This function inserts each element as it is read into sorted position
func insertSort(list *[]int, num int) {
	*list = append(*list, num)
	i := len((*list)) - 1
	for i > 0 && (*list)[i-1] > (*list)[i] {
		(*list)[i-1], (*list)[i] = (*list)[i], (*list)[i-1]
		i--
	}
}

// This compares the two lists and calculates the total difference between each element
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

// This will either add a new key to the map if it doesn't already exist and assign it to value 1
// or it will increment the value of that key this getting the count of each number
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

// This will multiply each element in the first list with its count in the map getting the similarity score
func compareMap(list *[]int, counts *map[int]int) int {
	simScore := 0
	for _, element := range *list {
		simScore += element * (*counts)[element]
	}
	return simScore
}
