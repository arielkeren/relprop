package types

type Content struct {
	SetSizes    []uint8
	Totals      []uint64
	Counts      map[string][]uint64
	Percentages map[string][]float64
}

type TestResults struct {
	CannotCheck uint16
	Correct     uint16
	Incorrect   uint16
}
