#include <inttypes.h>
#include <omp.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <time.h>

#include "properties.h"

#define MIN_SET_SIZE 1
#define MAX_SET_SIZE 5

void check_relations(uint8_t set_size, uint64_t count[NUMBER_OF_PROPERTIES],
                     uint64_t total_relations) {
    for (uint64_t relation = 0; relation < total_relations; relation++)
        for (uint8_t i = 0; i < NUMBER_OF_PROPERTIES; i++)
            count[i] += property_functions[i](relation, set_size);
}

void print_results(uint8_t set_size, uint64_t count[NUMBER_OF_PROPERTIES],
                   uint64_t total_relations, double elapsed) {
    printf("-------------------\n");
    printf("Set size: %" PRIu8 "\n", set_size);
    printf("Total relations: %" PRIu64 "\n", total_relations);
    printf("Time to check: %.3f seconds\n\n", elapsed);

    for (uint8_t i = 0; i < NUMBER_OF_PROPERTIES; i++) {
        printf("%s\n", property_names[i]);
        printf("Total: %" PRIu64 "\n", count[i]);
        printf("Percentage: %.3f%%\n\n",
               ((float)count[i] / total_relations) * 100);
    }
}

int main() {
    for (uint8_t set_size = MIN_SET_SIZE; set_size <= MAX_SET_SIZE;
         set_size++) {
        uint64_t total_relations = 1 << (set_size * set_size);
        uint64_t count[NUMBER_OF_PROPERTIES] = {0};

        clock_t start = clock();

        check_relations(set_size, count, total_relations);

        clock_t end = clock();
        float elapsed = (float)(end - start) / CLOCKS_PER_SEC;

        print_results(set_size, count, total_relations, elapsed);
    }

    return 0;
}
