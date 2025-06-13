package validation

import (
	"encoding/csv"
	"os"
	"strconv"
	"strings"
)

func ReadCSV() (map[string][]uint64, error) {
	file, err := os.Open("../results.csv")
	if err != nil {
		return nil, err
	}
	defer file.Close()

	reader := csv.NewReader(file)
	records, err := reader.ReadAll()
	if err != nil {
		return nil, err
	}

	header := records[0]
	data := records[1:]

	propToCount := make(map[string][]uint64)

	for _, row := range data {
		for i, col := range header {
			if !strings.HasSuffix(col, "_Count") {
				continue
			}

			prop := strings.TrimSuffix(col, "_Count")
			count, _ := strconv.ParseUint(row[i], 10, 64)
			propToCount[prop] = append(propToCount[prop], count)
		}
	}

	return propToCount, nil
}
