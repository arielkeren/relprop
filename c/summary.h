#ifndef SUMMARY_H
#define SUMMARY_H

#include <stdint.h>

#include "properties.h"

void write_csv_header(const char filename[]);

void append_results_to_csv(const char filename[], uint8_t set_size,
                           uint64_t count[NUMBER_OF_PROPERTIES],
                           uint64_t total_relations, double elapsed);

void print_results(uint8_t set_size, uint64_t count[NUMBER_OF_PROPERTIES],
                   uint64_t total_relations, double elapsed);

#endif
