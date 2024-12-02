package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Day 2")

	safeReportsCount := 0
	safeReportsCountDampened := 0

	file, _ := os.Open("data.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.Fields(scanner.Text())

		report := make([]int, 0, len(line))

		for _, elem := range line {
			num, _ := strconv.Atoi(elem)
			report = append(report, num)
		}

		if checkReport(&report) {
			//fmt.Printf("%v\n", report)
			safeReportsCount++
		} else if checkReportDampened(&report) {
			safeReportsCountDampened++
		}
	}

	fmt.Printf("Safe Reports: %v\n", safeReportsCount)

	totalSafeReports := safeReportsCount + safeReportsCountDampened

	fmt.Printf("Safe Reports Dampened: %v\n", totalSafeReports)

}

func checkReportDampened(report *[]int) bool {
	valid := false
	for i := range *report {
		modifiedReport := make([]int, 0, len(*report))
		modifiedReport = append(modifiedReport, (*report)[:i]...)
		modifiedReport = append(modifiedReport, (*report)[i+1:]...)
		//fmt.Println(modifiedReport)
		valid = checkReport(&modifiedReport)
		if valid {
			break
		}
	}
	return valid
}

func checkReport(report *[]int) bool {
	valid := true
	// Element 1 should be > element 0 to be increasing
	increasing := (*report)[1] - (*report)[0]
	// Early return if it's neither increasing or decreasing
	if increasing == 0 {
		return false
	}
	for i := 1; i < len(*report); i++ {
		if increasing > 0 {
			valid = checkIncreasing((*report)[i-1], (*report)[i])
		}
		if increasing < 0 {
			valid = checkDecreasing((*report)[i-1], (*report)[i])
		}
		// Early exit if not valid
		if !valid {
			break
		}
	}
	return valid
}

func checkIncreasing(elem1 int, elem2 int) bool {
	if elem2-elem1 < 1 || elem2-elem1 > 3 {
		return false
	}
	return true
}

func checkDecreasing(elem1 int, elem2 int) bool {
	if elem1-elem2 < 1 || elem1-elem2 > 3 {
		return false
	}
	return true
}
