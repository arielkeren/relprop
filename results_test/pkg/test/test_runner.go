package test

import (
	"fmt"
	"relprop/pkg/types"
)

func RunTotalRelationsTest(totals []uint64, setSizes []uint8) types.TestResults {
	results := types.TestResults{}

	for i := 0; i < len(totals) && i < len(setSizes); i++ {
		total := totals[i]
		setSize := setSizes[i]

		expectedTotal := GetTotalRelations(setSize)

		if total == expectedTotal {
			results.Correct++
		} else {
			fmt.Println("Total relations for set size", setSize, "is incorrect: found", total, ", expected", expectedTotal)
			results.Incorrect++
		}
	}

	return results
}

func RunPropertyTests[T uint64 | float64](propToValue map[string][]T, setSizes []uint8) types.TestResults {
	results := types.TestResults{}

	for prop, values := range propToValue {
		for i := 0; i < len(values) && i < len(setSizes); i++ {
			csvValue := values[i]
			setSize := setSizes[i]

			var expectedValue T
			var isCheckable bool

			switch any(csvValue).(type) {
			case uint64:
				var count uint64
				count, isCheckable = GetCount(setSize, prop)
				expectedValue = T(count)
			case float64:
				var pct float64
				pct, isCheckable = GetPercentage(setSize, prop)
				expectedValue = T(pct)
			}

			if !isCheckable {
				results.CannotCheck++
				continue
			}

			if csvValue == expectedValue {
				results.Correct++
			} else {
				results.Incorrect++
				fmt.Println(prop, "(set size", setSize, "): Found", csvValue, ", but should be", expectedValue)
			}
		}
	}

	return results
}
