#ifndef PARSER_H
#define PARSER_H

#include <stdint.h>

void get_set_size(int argc, char *argv[], uint8_t *min_size, uint8_t *max_size);
void print_usage(const char program_name[]);
uint8_t parse_uint8(const char arg[]);

#endif
