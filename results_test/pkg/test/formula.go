package test

func pow(base, exponent uint64) uint64 {
	if base == 2 {
		return 1 << exponent
	}

	result := uint64(1)
	for i := uint64(0); i < exponent; i++ {
		result *= base
	}
	return result
}

func GetCount(setSize uint8, property string) (uint64, bool) {
	n := uint64(setSize)

	switch property {
	case "Antisymmetry":
		return pow(3, n*(n-1)/2) * pow(2, n), true
	case "Asymmetry":
		return pow(3, n*(n-1)/2), true
	case "Coreflexivity":
		return pow(2, n), true
	case "Irreflexivity":
		return pow(2, n*n-n), true
	case "Reflexivity":
		return pow(2, n*n-n), true
	case "Symmetry":
		return pow(2, n*(n+1)/2), true
	case "Totality":
		return pow(3, n*(n-1)/2), true
	case "Trichotomy":
		return pow(3, n*(n-1)/2) * pow(2, n), true
	default:
		return 0, false
	}
}

func GetPercentage(setSize uint8, property string) (float64, bool) {
	count, isCheckable := GetCount(setSize, property)

	if !isCheckable {
		return 0, false
	}

	total := GetTotalRelations(setSize)
	pct := 100 * float64(count) / float64(total)

	return pct, true
}

func GetTotalRelations(setSize uint8) uint64 {
	n := uint64(setSize)
	return pow(2, n*n)
}
