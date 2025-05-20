#ifndef CHECKER_H
#define CHECKER_H

#include <stdint.h>

#include "properties.h"

void check_relations(uint8_t set_size, uint64_t count[NUMBER_OF_PROPERTIES],
                     uint64_t total_relations);

void start_checking(const char filename[], uint8_t min_set_size,
                    uint8_t max_set_size);

#endif
