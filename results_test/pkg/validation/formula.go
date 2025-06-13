package validation

func pow(a, b uint64) uint64 {
	if a == 2 {
		return 1 << b
	}

	result := uint64(1)
	for i := uint64(0); i < b; i++ {
		result *= a
	}
	return result
}

func ApplyFormula(n uint64, property string) uint64 {
	switch property {
	case "Antisymmetry":
		return pow(3, n*(n-1)/2) * pow(2, n)
	case "Asymmetry":
		return pow(3, n*(n-1)/2)
	case "Coreflexivity":
		return pow(2, n)
	case "Irreflexivity":
		return pow(2, n*n-n)
	case "Reflexivity":
		return pow(2, n*n-n)
	case "Symmetry":
		return pow(2, n*(n+1)/2)
	case "Totality":
		return pow(3, n*(n-1)/2)
	case "Trichotomy":
		return pow(3, n*(n-1)/2) * pow(2, n)
	default:
		return 0
	}
}
