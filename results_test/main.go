package main

import (
	"fmt"
	"results_test/pkg/validation"
)

func main() {
	propToCount, err := validation.ReadCSV()
	if err != nil {
		fmt.Println("Error reading CSV:", err)
		return
	}

	cannotCheck, correct, incorrect := validation.RunTests(propToCount)
	validation.PrintSummary(cannotCheck, correct, incorrect)
}
