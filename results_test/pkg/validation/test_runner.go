package validation

import "fmt"

func PrintSummary(cannotCheck, correct, incorrect uint16) {
	fmt.Println("\n---- Results Summary ----")
	fmt.Println("Correct:", correct)
	fmt.Println("Incorrect:", incorrect)
	fmt.Println("Cannot check:", cannotCheck)
	fmt.Println()

	if incorrect == 0 {
		fmt.Println("ALL TESTS PASSED")
	} else {
		fmt.Println("SOME TESTS FAILED")
	}
	fmt.Println("-------------------------")
}

func RunTests(props map[string][]uint64) (uint16, uint16, uint16) {
	cannotCheck := uint16(0)
	correct := uint16(0)
	incorrect := uint16(0)

	for prop, vals := range props {
		for setSize, data := range vals {
			formulaValue := ApplyFormula(uint64(setSize), prop)
			if formulaValue == 0 {
				cannotCheck++
				continue
			}
			csvValue := data
			if csvValue == formulaValue {
				correct++
			} else {
				incorrect++
				fmt.Println(prop, "(set size", setSize, "): Found", csvValue, ", but should be", formulaValue)
			}
		}
	}

	return cannotCheck, correct, incorrect
}
