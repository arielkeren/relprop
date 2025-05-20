#include "parser.h"

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

uint8_t parse_uint8(const char *arg) {
    errno = 0;
    char *end;
    unsigned long val = strtoul(arg, &end, 10);

    if (errno != 0 || *end != '\0' || val > UINT8_MAX) {
        fprintf(stderr, "Invalid input for uint8_t: '%s'\n", arg);
        exit(1);
    }

    return (uint8_t)val;
}

void get_set_size(int argc, char *argv[], uint8_t *min_size,
                  uint8_t *max_size) {
    *min_size = DEFAULT_MIN_SET_SIZE;
    *max_size = DEFAULT_MAX_SET_SIZE;

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "-min") == 0 && i + 1 < argc) {
            *min_size = parse_uint8(argv[i + 1]);
            i++;
        } else if (strcmp(argv[i], "-max") == 0 && i + 1 < argc) {
            *max_size = parse_uint8(argv[i + 1]);
            i++;
        } else {
            fprintf(stderr, "Unknown or incomplete argument: %s\n", argv[i]);
            fprintf(stderr, "Usage: %s [-min N] [-max M]\n", argv[0]);
            exit(1);
        }
    }

    if (*min_size < 1 || *max_size < *min_size) {
        fprintf(stderr, "Invalid set size range: min = %d, max = %d\n",
                *min_size, *max_size);
        exit(1);
    }
}
