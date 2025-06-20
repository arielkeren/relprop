#include "checker.h"

#include <inttypes.h>
#include <omp.h>
#include <stdint.h>
#include <stdio.h>
#include <time.h>

#include "constants.h"
#include "properties.h"
#include "summary.h"

void start_checking(const char filename[], uint8_t min_set_size,
                    uint8_t max_set_size) {
    for (uint8_t set_size = min_set_size; set_size <= max_set_size;
         set_size++) {
        printf("Checking relations on set size: %" PRIu8 "\n", set_size);

        uint64_t total_relations = 1ULL << (set_size * set_size);
        uint64_t count[NUMBER_OF_PROPERTIES] = {0};

        clock_t start = clock();

        check_relations(set_size, total_relations, count);

        clock_t end = clock();
        float elapsed = (float)(end - start) / CLOCKS_PER_SEC;

        append_results_to_csv(filename, set_size, count, total_relations,
                              elapsed);
    }
}

void check_relations(uint8_t set_size, uint64_t total_relations,
                     uint64_t count[NUMBER_OF_PROPERTIES]) {
#pragma omp parallel
    {
        uint64_t thread_counts[NUMBER_OF_PROPERTIES] = {0};
        uint8_t relation[MAX_SET_SIZE];

#pragma omp for schedule(static)
        for (uint64_t idx = 0; idx < total_relations; idx++) {
            uint64_t val = idx;

            for (uint8_t i = 0; i < set_size; i++) {
                relation[i] = val & ((1 << set_size) - 1);
                val >>= set_size;
            }

            for (uint8_t i = 0; i < NUMBER_OF_PROPERTIES; i++)
                thread_counts[i] += PROPERTY_FUNCTIONS[i](relation, set_size);
        }

#pragma omp critical
        for (uint8_t i = 0; i < NUMBER_OF_PROPERTIES; i++)
            count[i] += thread_counts[i];
    }
}
