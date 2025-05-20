#ifndef PARSER_H
#define PARSER_H

#include <inttypes.h>

#define DEFAULT_MIN_SET_SIZE 1
#define DEFAULT_MAX_SET_SIZE 5

uint8_t parse_uint8(const char *arg);

void get_set_size(int argc, char *argv[], uint8_t *min_size, uint8_t *max_size);

#endif
