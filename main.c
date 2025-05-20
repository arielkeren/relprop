#include <inttypes.h>
#include <omp.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <time.h>

#include "checker.h"
#include "parser.h"
#include "properties.h"
#include "summary.h"

int main(int argc, char *argv[]) {
    write_csv_header("results.csv");

    uint8_t min_set_size;
    uint8_t max_set_size;
    get_set_size(argc, argv, &min_set_size, &max_set_size);

    start_checking(min_set_size, max_set_size);

    return 0;
}
