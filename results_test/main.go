package main

import (
	"fmt"
	"os"
	"relprop/pkg/reader"
	"relprop/pkg/summary"
	"relprop/pkg/test"
)

func main() {
	path := "../results.csv"
	fmt.Println("Reading CSV file")

	content, err := reader.ReadCSV(path)
	if err != nil {
		fmt.Fprintln(os.Stderr, "Error reading CSV:", err)
		return
	}

	fmt.Println("CSV file read successfully - Starting tests")

	totalResults := test.RunTotalRelationsTest(content.Totals, content.SetSizes)
	countResults := test.RunPropertyTests(content.Counts, content.SetSizes)
	percentageResults := test.RunPropertyTests(content.Percentages, content.SetSizes)
	summary.PrintSummary(totalResults, countResults, percentageResults)
}
