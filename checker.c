#include "checker.h"

#include <stdint.h>
#include <time.h>

#include "properties.h"
#include "summary.h"

void check_relations(uint8_t set_size, uint64_t count[NUMBER_OF_PROPERTIES],
                     uint64_t total_relations) {
    for (uint64_t relation = 0; relation < total_relations; relation++)
        for (uint8_t i = 0; i < NUMBER_OF_PROPERTIES; i++)
            count[i] += property_functions[i](relation, set_size);
}

void start_checking(const char filename[], uint8_t min_set_size,
                    uint8_t max_set_size) {
    for (uint8_t set_size = min_set_size; set_size <= max_set_size;
         set_size++) {
        uint64_t total_relations = 1 << (set_size * set_size);
        uint64_t count[NUMBER_OF_PROPERTIES] = {0};

        clock_t start = clock();

        check_relations(set_size, count, total_relations);

        clock_t end = clock();
        float elapsed = (float)(end - start) / CLOCKS_PER_SEC;

        print_results(set_size, count, total_relations, elapsed);
        append_results_to_csv(filename, set_size, count, total_relations,
                              elapsed);
    }
}
