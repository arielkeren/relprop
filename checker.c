#include "checker.h"

#include <omp.h>
#include <stdbool.h>
#include <stdint.h>
#include <time.h>

#include "properties.h"
#include "summary.h"

void check_relations(uint8_t set_size, uint64_t count[NUMBER_OF_PROPERTIES]) {
    uint64_t total = 1ULL << (set_size * set_size);

#pragma omp parallel
    {
        uint64_t thread_counts[NUMBER_OF_PROPERTIES] = {0};
        uint8_t relation[8];

#pragma omp for schedule(static)
        for (uint64_t idx = 0; idx < total; idx++) {
            for (uint8_t i = 0, val = idx; i < set_size; i++) {
                relation[i] = val & ((1 << set_size) - 1);
                val >>= set_size;
            }
            for (uint8_t i = 0; i < NUMBER_OF_PROPERTIES; i++)
                thread_counts[i] += property_functions[i](relation, set_size);
        }

#pragma omp critical
        for (uint8_t i = 0; i < NUMBER_OF_PROPERTIES; i++)
            count[i] += thread_counts[i];
    }
}

void start_checking(const char filename[], uint8_t min_set_size,
                    uint8_t max_set_size) {
    for (uint8_t set_size = min_set_size; set_size <= max_set_size;
         set_size++) {
        uint64_t total_relations = 1ULL << (set_size * set_size);
        uint64_t count[NUMBER_OF_PROPERTIES] = {0};

        clock_t start = clock();

        check_relations(set_size, count);

        clock_t end = clock();
        float elapsed = (float)(end - start) / CLOCKS_PER_SEC;

        print_results(set_size, count, total_relations, elapsed);
        append_results_to_csv(filename, set_size, count, total_relations,
                              elapsed);
    }
}
