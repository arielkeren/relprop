package summary

import (
	"fmt"
	"relprop/pkg/types"
)

func PrintSummary(totalResults, countResults, percentageResults types.TestResults) {
	results := []types.TestResults{totalResults, countResults, percentageResults}
	testNames := []string{"Total Relations", "Property Count", "Property Percentage"}

	if totalResults.Incorrect == 0 && countResults.Incorrect == 0 && percentageResults.Incorrect == 0 {
		fmt.Println("\n>>> ALL TESTS PASSED >>>")
	} else {
		fmt.Println("\n>>> SOME TESTS FAILED >>>")
	}

	for i := range results {
		printTestResults(results[i], testNames[i])
	}
}

func printTestResults(results types.TestResults, testName string) {
	fmt.Println("\n\n----", testName, "Test Summary ----")
	fmt.Println("Correct:", results.Correct)
	fmt.Println("Incorrect:", results.Incorrect)
	fmt.Println("Cannot Check:", results.CannotCheck)
	fmt.Println()

	if results.Incorrect == 0 {
		fmt.Println("TEST PASSED")
	} else {
		fmt.Println("TEST FAILED")
	}

	fmt.Println("-------------------------")
}
