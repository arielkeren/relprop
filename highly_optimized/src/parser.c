#include "parser.h"

#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "constants.h"

uint8_t parse_uint8(const char arg[]) {
    errno = 0;
    char *end;
    unsigned long val = strtoul(arg, &end, 10);

    if (errno != 0 || *end != '\0' || val > UINT8_MAX) {
        fprintf(stderr, "Invalid set size value: '%s'\n", arg);
        exit(1);
    }

    return (uint8_t)val;
}

void get_set_size(int argc, char *argv[], uint8_t *min_size,
                  uint8_t *max_size) {
    *min_size = DEFAULT_MIN_SET_SIZE;
    *max_size = DEFAULT_MAX_SET_SIZE;

    for (int i = 1; i < argc; i++) {
        if ((strcmp(argv[i], "--min") == 0 || strcmp(argv[i], "-m") == 0) &&
            i + 1 < argc) {
            *min_size = parse_uint8(argv[i + 1]);
            i++;
        } else if ((strcmp(argv[i], "--max") == 0 ||
                    strcmp(argv[i], "-M") == 0) &&
                   i + 1 < argc) {
            *max_size = parse_uint8(argv[i + 1]);
            i++;
        } else {
            fprintf(stderr, "Unknown or incomplete argument: %s\n", argv[i]);
            fprintf(stderr, "Usage: %s --min 2 --max 5\n", argv[0]);
            fprintf(stderr, "Or:    %s -m 2 -M 5\n", argv[0]);
            exit(1);
        }
    }

    if (*max_size > MAX_SET_SIZE) {
        fprintf(stderr, "Maximum set size cannot exceed %d\n", MAX_SET_SIZE);
        exit(1);
    }

    if (*min_size > *max_size) {
        fprintf(stderr,
                "The minimum set size, %d, cannot be bigger than the maximum "
                "set size, %d\n",
                *min_size, *max_size);
        exit(1);
    }
}
