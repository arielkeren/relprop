#ifndef CHECKER_H
#define CHECKER_H

#include <stdint.h>

#include "constants.h"

void start_checking(const char filename[], uint8_t min_set_size,
                    uint8_t max_set_size);

void check_relations(uint8_t set_size, uint64_t total_relations,
                     uint64_t count[NUMBER_OF_PROPERTIES]);

#endif
