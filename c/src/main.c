#include <stdint.h>

#include "checker.h"
#include "parser.h"
#include "summary.h"

int main(int argc, char *argv[]) {
    const char filename[] = "results.csv";
    write_csv_header(filename);

    uint8_t min_set_size;
    uint8_t max_set_size;
    get_set_size(argc, argv, &min_set_size, &max_set_size);

    start_checking(filename, min_set_size, max_set_size);

    return 0;
}
