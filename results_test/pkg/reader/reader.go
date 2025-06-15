package reader

import (
	"encoding/csv"
	"errors"
	"os"
	"relprop/pkg/types"
	"strconv"
	"strings"
)

func ReadCSV(path string) (types.Content, error) {
	header, data, err := openCSV(path)
	if err != nil {
		return types.Content{}, err
	}

	content := types.Content{
		SetSizes:    []uint8{},
		Totals:      []uint64{},
		Counts:      make(map[string][]uint64),
		Percentages: make(map[string][]float64),
	}

	for _, row := range data {
		for i, colName := range header {
			if err = handleValue(&content, colName, row[i]); err != nil {
				return types.Content{}, err
			}
		}
	}

	return content, nil
}

func openCSV(path string) ([]string, [][]string, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, nil, err
	}
	defer file.Close()

	reader := csv.NewReader(file)
	records, err := reader.ReadAll()
	if err != nil {
		return nil, nil, err
	}

	return records[0], records[1:], nil
}

func handleValue(content *types.Content, colName string, value string) error {
	switch colName {
	case "Set":
		setSize, err := strconv.ParseUint(value, 10, 8)
		if err != nil {
			return err
		}

		content.SetSizes = append(content.SetSizes, uint8(setSize))
		return nil

	case "Total":
		total, err := strconv.ParseUint(value, 10, 64)
		if err != nil {
			return err
		}

		content.Totals = append(content.Totals, total)
		return nil

	case "Time":
		_, err := strconv.ParseFloat(value, 64)
		return err
	}

	if strings.HasSuffix(colName, "_Count") {
		count, err := strconv.ParseUint(value, 10, 64)
		if err != nil {
			return err
		}

		prop := strings.TrimSuffix(colName, "_Count")
		content.Counts[prop] = append(content.Counts[prop], count)

		return nil
	}

	if strings.HasSuffix(colName, "_Pct") {
		pct, err := strconv.ParseFloat(value, 64)
		if err != nil {
			return err
		}

		prop := strings.TrimSuffix(colName, "_Pct")
		content.Percentages[prop] = append(content.Percentages[prop], pct)

		return nil
	}

	return errors.New("Unknown column name: " + colName)
}
