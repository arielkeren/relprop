#ifndef CONSTANTS_H
#define CONSTANTS_H

#include <stdbool.h>
#include <stdint.h>

#define DEFAULT_MIN_SET_SIZE 0
#define DEFAULT_MAX_SET_SIZE 5
#define MAX_SET_SIZE 8
#define NUMBER_OF_PROPERTIES 17

extern const char *PROPERTY_NAMES[NUMBER_OF_PROPERTIES];
extern bool (*const PROPERTY_FUNCTIONS[NUMBER_OF_PROPERTIES])(uint8_t[],
                                                              uint8_t);

#endif
